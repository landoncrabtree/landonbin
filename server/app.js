var createError = require("http-errors");
var express = require("express");
var path = require("path");
var cookieParser = require("cookie-parser");
var logger = require("morgan");

// additional modules
var subdomain = require("express-subdomain");
var apiRouter = require("./routes/api");

var app = express();

// view engine setup
app.set("views", path.join(__dirname, "views"));
app.set("view engine", "pug");

app.use(logger("dev"));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());

// Static files
app.use(express.static(path.join(__dirname, "public")));

app.use(subdomain("*.api", apiRouter));
app.use("/", apiRouter);

// catch any other routes and return 404
app.use(function (req, res, next) {
    next(createError(404));
});

// error handler
app.use(function (err, req, res, next) {
    // set locals, only providing error in development
    const development = process.env.NODE_ENV === "development";
    const requestedResource = req.originalUrl;
    const ipAddr = req.headers["x-real-ip"];

    if (development) {
        return res.status(err.status || 500).json({
            title: "An error occurred",
            error: err,
            requestedResource: requestedResource,
            ipAddr: ipAddr,
        });
    } else {
        return res.status(err.status || 500).json({
            title: "An error occurred",
            error: {},
            requestedResource: requestedResource,
            ipAddr: ipAddr,
        });
    }
});

const port = process.env.PORT || 3000;
app.listen(port, () => console.log(`App listening on port ${port}!`));

module.exports = app;
