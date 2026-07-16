**CYBERDECK: System Diagnostics & Maintenance**

CYBERDECK is a high-performance, web-based diagnostic suite designed for real-time hardware oversight and system maintenance. Built with a robust Rust/Axum backend and a lightweight, reactive frontend, it provides a secure interface to monitor system health and generate forensic snapshots of your machine's performance.

**Core Features:**

- **Deep Diagnostics:** Monitor CPU/GPU thermal output, battery health, and critical service status in real-time.

- **Forensic Archival:** An integrated compression engine capable of packaging diagnostic logs, hardware reports, and raw data into `zip`, `tar`, `gzip`, or `7z` archives with a single command.

- **Custom Theming:** Fully skinnable interface architecture, allowing users to apply custom CSS themes to match their environment.

- **Low Overhead:** Built on top of asynchronous Rust, ensuring minimal system impact even while performing intensive monitoring tasks.

- **Accessible Design:** Utilizes HeadlessUI and Alpine.js for a responsive, keyboard-accessible interaction model.
