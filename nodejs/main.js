const io = require("socket.io-client");

// Replace with your actual token
const token = "ddrLdsHA";
// const server = "https://zarena-dev1.zinza.com.vn";
const server = "http://localhost:3000";
// Connect to the socket server
const socket = io(server, {
  auth: { token: token },
});

// Handle connection error
socket.on("connect_error", (err) => {
  console.error("Connection error:", err);
});

// Emit a 'join' event with data
socket.emit("join", { ds: "ds" }, (response) => {
  console.log("Join response:", response); // Optional: Handle response
});

// Keep the connection alive
const keepAlive = setInterval(() => {
  console.log("Connection is alive...");
}, 60000); // Log every minute

// Handle shutdown signal (Ctrl+C)
process.on("SIGINT", () => {
  console.log("Disconnecting...");
  clearInterval(keepAlive); // Stop the keep-alive interval
  socket.disconnect(); // Disconnect the socket
  process.exit();
});
