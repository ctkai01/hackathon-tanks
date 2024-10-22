const express = require("express");
const http = require("http");
const { Server } = require("socket.io");

const app = express();
const port = 3000;

// Create an HTTP server
const server = http.createServer(app);

// Set up Socket.io
const io = new Server(server);

// Serve a simple HTML page for testing
app.get("/", (req, res) => {
  res.sendFile(__dirname + "/index.html");
});

// Handle a connection event
io.on("connection", (socket) => {
  console.log("A user connected:", socket.id);
socket.emit("message", `You have connect ${socket.id}`);
  // Handle join event
  socket.on("join", (room) => {
    console.log("JOIN")
    socket.join(room);
    console.log(`User ${socket.id} joined room: ${room}`);
    // Optionally, notify the user that they've joined the room
    // socket.emit("message", `You have joined room: ${room}`);
  });

  // Handle disconnection
  socket.on("disconnect", () => {
    console.log("User disconnected:", socket.id);
  });
});

// Start the server
server.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});
