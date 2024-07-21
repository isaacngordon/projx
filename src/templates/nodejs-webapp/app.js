// app
const express = require('express');
const path = require('path');
const app = express();
const PORT = process.env.PORT || 3000;

// routers
const apiRouter = require('./routes/api/index');

// Serve static files from the "public" directory
app.use(express.static(path.join(__dirname, 'public')));

// Index route
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

// API route
app.use('/api', apiRouter);

// 404 route
app.use((req, res, next) => {
    res.status(404).sendFile(path.join(__dirname, 'views', '404.html'));
});

app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});
