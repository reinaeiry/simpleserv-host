# SimpleServ Host

## ⚙️ Intended for Use with [SimpleServ Client](https://github.com/reinaeiry/simpleserv-client)

`simpleserv-host` is designed to work alongside the [SimpleServ Client](https://github.com/reinaeiry/simpleserv-client) for a seamless remote system control experience. The **host** application runs on your local machine, providing a web-based control panel for interacting with the **client** running on the target machine. 

![Remote System Status Image](https://cdn.discordapp.com/attachments/1273814692845981777/1371068558209060874/image.png?ex=6821ca99&is=68207919&hm=cf12911b818c8783e6b781a3f727656508e3a43c35d6579d0d21d1adc9447d5c&)

---

## 🚀 Overview

`simpleserv-host` is the server-side application that provides a web-based control panel for managing and interacting with a remote system. It connects to the client-side application (`simpleserv-client`) to display system status, execute commands, and provide power management options (Power Off, Restart).
It serves as the dashboard interface for monitoring system performance, running commands, and triggering actions remotely.

## 🔧 Installation

1. **Clone the repository**:

   Clone the `simpleserv-host` repository to your local machine:

   ```
   git clone https://github.com/reinaeiry/simpleserv-host.git
   cd simpleserv-host
   ```

2. **Build the project**:

   Use `cargo` to build the project:

   ```
   cargo build --release
   ```

3. **Run the project**:

   Start the server by running:

   ```
   cargo run
   ```

   The server will be accessible at `http://localhost:6969` by default.

---

## 🌐 Features

### 1. **Remote System Status**
   - Displays important system information like hostname, CPU usage, memory usage, temperature, and uptime.
   
### 2. **Power Management**
   - **Power Off**: Allows you to power off the target system remotely.
   - **Restart**: Allows you to restart the target system remotely.
   
### 3. **Execute Commands**
   - You can run shell commands on the target system and view their output in real time.

### 4. **Interactive Dashboard**
   - Provides a web interface to visualize system stats such as CPU usage and memory utilization through charts.

---

## 🧰 Intended Use

This host application is intended to be used in conjunction with the `simpleserv-client`. The **client** runs on the target machine, gathering system data and executing commands, while the **host** is the interface for interacting with the system remotely. You can access it via a web browser on your local machine.

---

## 🌐 Accessing the Panel

- To access the web control panel, open your browser and visit:  
  `http://localhost:6969`

- **Note**: Make sure the `simpleserv-client` is running on the target machine and on the same wifi network as the host.
