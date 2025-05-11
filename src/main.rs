
use axum::{routing::get, Router, response::Html, extract::Query};
use std::net::SocketAddr;
use tokio::process::Command;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/status", get(fetch_status))
        .route("/exec", get(exec_command));

    let addr = SocketAddr::from(([0, 0, 0, 0], 6969));
    println!("Control panel running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn dashboard() -> Html<&'static str> {
    Html(r#"
  <!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>Status Dashboard</title>
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
  <style>
    body { font-family: sans-serif; background: #111; color: #eee; padding: 20px; }
    .stat { margin: 10px 0; }
    canvas { background: #222; border-radius: 8px; padding: 10px; max-width: 1000px; height: 450px; }
    .sticky-header {
      position: sticky;
      top: 0;
      background-color: #111;
      padding: 20px 0;
      z-index: 10;
      border-radius: 8px 8px 0 0;
      color: #eee;
      text-align: left;
      font-size: 24px;
      font-weight: bold;
      box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
    }
    .button-bar { 
      position: sticky; 
      top: 50px; 
      background-color: #111; 
      padding: 15px; 
      z-index: 10; 
      border-radius: 8px 8px 0 0; 
      display: flex;
      gap: 10px;
      box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5); 
    }
    button {
      margin-right: 10px;
      padding: 10px 15px;
      border: none;
      border-radius: 5px;
      background-color: #333;
      color: white;
      cursor: pointer;
    }
    button:hover { background-color: #555; }
    #commandOutput { margin-top: 20px; color: lime; white-space: pre-line; }
  </style>
</head>
<body>
  <div class="sticky-header">Remote System Status</div>
  <div class="button-bar">
    <button onclick="alert('Power Off triggered')">⚡ Power Off</button>
    <button onclick="alert('Restart triggered')">🔄 Restart</button>
    <button onclick="runCommand()">💻 Run Command</button>
  </div>

  <div id="stats"></div>
  <canvas id="cpuChart"></canvas>
  <canvas id="memChart"></canvas>
  <div id="commandOutput"></div> <!-- Display command output here -->

  <script>
    const cpuData = [];
    const memData = [];
    const labels = [];

    const cpuChart = new Chart(document.getElementById('cpuChart'), {
      type: 'line',
      data: {
        labels,
        datasets: [{ label: 'CPU %', data: cpuData, borderColor: 'lime', fill: false }]
      },
      options: { 
        responsive: false, 
        maintainAspectRatio: false,
        scales: {
          y: {
            suggestedMax: 100,  // Set max for CPU chart to 100% at all times
          }
        }
      }
    });

    const memChart = new Chart(document.getElementById('memChart'), {
      type: 'line',
      data: {
        labels,
        datasets: [{ label: 'Used Memory (GB)', data: memData, borderColor: 'skyblue', fill: false }]
      },
      options: { 
        responsive: false, 
        maintainAspectRatio: false,
        scales: {
          y: {
            suggestedMax: 10,  // This will be dynamically set from the backend
          }
        }
      }
    });

    function formatUptime(seconds) {
      const d = Math.floor(seconds / (3600*24));
      const h = Math.floor(seconds % (3600*24) / 3600);
      const m = Math.floor(seconds % 3600 / 60);
      const s = Math.floor(seconds % 60);
      return `${d}d ${h}h ${m}m ${s}s`;
    }

    async function refresh() {
      const res = await fetch('/status');
      const data = await res.json();

      const memUsedGB = (data.used_memory / 1024 / 1024 / 1024).toFixed(2);
      const memTotalGB = (data.total_memory / 1024 / 1024 / 1024).toFixed(2);

      document.getElementById('stats').innerHTML = `        
        <div class="stat">🖥️ Hostname: <strong>${data.hostname}</strong></div>
        <div class="stat">⏱️ Uptime: <strong>${formatUptime(data.uptime)}</strong></div>
        <div class="stat">🧠 Memory: <strong>${memUsedGB} / ${memTotalGB} GB</strong></div>
        <div class="stat">⚙️ CPU Usage: <strong>${Math.round(data.cpu_usage)}%</strong></div>
        <div class="stat">🌡️ Temperature: <strong>${data.temperature ?? 'N/A'}</strong></div>
      `;

      const timeLabel = new Date().toLocaleTimeString();
      labels.push(timeLabel);
      cpuData.push(data.cpu_usage);
      memData.push(parseFloat(memUsedGB));

      if (labels.length > 30) {
        labels.shift(); cpuData.shift(); memData.shift();
      }

      // Update CPU chart
      cpuChart.data.labels = labels;
      cpuChart.data.datasets[0].data = cpuData;
      cpuChart.update();

      // Update Memory chart
      memChart.data.labels = labels;
      memChart.data.datasets[0].data = memData;
      memChart.options.scales.y.suggestedMax = parseFloat(memTotalGB);  // Set memory chart max dynamically
      memChart.update();
    }

    function runCommand() {
      const cmd = prompt("Enter command to run:");
      if (cmd) {
        fetch(`/exec?cmd=${encodeURIComponent(cmd)}`)
          .then(res => res.text()) // Get plain text output
          .then(data => {
            document.getElementById('commandOutput').innerText = `Command Output:\n${data}`;
          })
          .catch(err => {
            document.getElementById('commandOutput').innerText = "Error executing command.";
          });
      }
    }

    setInterval(refresh, 2000);
    refresh();
  </script>
</body>
</html>
    "#)
}

async fn fetch_status() -> Html<String> {
    let Ok(response) = reqwest::get("http://192.168.1.105:3030/status").await else {
        return Html("{\"error\":\"Failed to reach client\"}".to_string());
    };

    let Ok(body) = response.text().await else {
        return Html("{\"error\":\"Invalid response\"}".to_string());
    };

    Html(body)
}

async fn exec_command(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    if let Some(cmd) = params.get("cmd") {
        let output = Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .await;

        let output_str = match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                if !stdout.is_empty() {
                    stdout 
                } else if !stderr.is_empty() {
                    stderr
                } else {
                    "No output".to_string()
                }
            },
            Err(e) => format!("Error executing command: {}", e),
        };

        Html(output_str)
    } else {
        Html("No command provided".to_string())
    }
}
