// routes/api.js
const express = require('express');
const router = express.Router();
const helloRouter = require("./hello")

// API route
router.get('/', (req, res) => {
	res.send("Api index"); 
});
router.use('/hello', helloRouter);

module.exports = router;
