var express = require("express");
var router = express.Router();
var createError = require("http-errors");
var mongoose = require("mongoose");
var base64 = require("base-64");
var hljs = require("highlight.js");
var path = require("path");

const API_KEYS = [
	"my-secret-api-key",
	"my-other-secret-api-key",
	"my-api-key-i-gave-to-my-friend"
]

const MONGO_URI = "mongodb://user:password@localhost:27017/pastes"

// Connect to MongoDB
mongoose.connect(MONGO_URI, {
    useNewUrlParser: true,
    useUnifiedTopology: true,
});
const db = mongoose.connection;
db.on("error", console.error.bind(console, "MongoDB connection error:"));
db.once("open", () => {
    console.log("Connected to MongoDB");
});

// Define a schema for the paste data
const pasteSchema = new mongoose.Schema({
    content: String,
    expiry: { type: Date, default: null },
});

pasteSchema.index(
    { expiry: 1 },
    {
        expireAfterSeconds: 0,
        partialFilterExpression: { expiry: { $ne: null } },
    }
);

const Paste = mongoose.model("Paste", pasteSchema);

router.post("/pastes", (req, res, next) => {
    const authorization = req.headers["x-api-key"];
	// check if authorization is in API_KEYS
    if (API_KEYS.includes(authorization)) {
        return res.status(401).json({
            status: 401,
            error: "Unauthorized",
        });
    }
    let { content, expiry } = req.body;
    var expiresAt = new Date();
    if (expiry == "Never") {
        expiresAt = null;
    } else {
        try {
            expiresAt.setMilliseconds(
                expiresAt.getMilliseconds() + parseDuration(expiry)
            );
        } catch (e) {
            return res.status(400).json({
                status: 400,
                error: e.message,
            });
        }
    }
    const paste = new Paste({
        content: content,
        expiry: expiresAt,
    });

    paste
        .save()
        .then((paste) => {
            return res.status(200).json({
                status: 200,
                id: paste._id,
                expiry: paste.expiry,
                url: `https://api.example.com/pastes/${paste._id}`,
            });
        })
        .catch((err) => {
            return res.status(500).json({
                status: 500,
                error: err.message,
            });
    	});
});

router.get("/pastes/raw/:id", (req, res, next) => {
    const id = req.params.id;
    Paste.findById(id)
        .then((paste) => {
            const text = base64.decode(paste.content);
            return res.render("raw_paste", {
                title: text.split("\n")[0],
                content: text.replace(/</g, "&lt;").replace(/>/g, "&gt;"),
                id: id,
            });
        })
        .catch((err) => {
            return res.status(500).json({
                status: 404,
                error: "Paste not found",
            });
        });
});

router.get("/pastes/:id", (req, res, next) => {
    let id = req.params.id;
    let ext = path.extname(id);
    // remove extension from id and remove "." from extension
    if (ext) {
        id = id.split(ext)[0];
        ext = ext.split(".")[1];
    }

    Paste.findById(id)
        .then((paste) => {
            const text = base64.decode(paste.content);
            if (ext) {
                try {
                    var syntaxHighlight = hljs.highlight(text, {
                        language: ext,
                    }).value;
                } catch (e) {
                    // fallback to auto
                    var syntaxHighlight = hljs.highlightAuto(text).value;
                }
            } else {
                var syntaxHighlight = hljs.highlightAuto(text).value;
            }
            let expiry = paste.expiry ? paste.expiry.toLocaleString() : "Never";
            return res.render("paste", {
                title: text.split("\n")[0],
                content: syntaxHighlight,
                expiry: expiry,
                id: id,
            });
        })
        .catch((err) => {
            return res.status(500).json({
                status: 404,
                error: "Paste not found",
            });
        });
});

function parseDuration(duration) {
    const regex = /^(\d+)([dhms])$/;
    const match = duration.match(regex);

    if (!match) {
        throw new Error("Invalid duration format");
    }

    const value = parseInt(match[1]);
    const unit = match[2];

    switch (unit) {
        case "d": // days
            return value * 24 * 60 * 60 * 1000;
        case "h": // hours
            return value * 60 * 60 * 1000;
        case "m": // minutes
            return value * 60 * 1000;
        case "s": // seconds
            return value * 1000;
        default:
            throw new Error("Invalid duration unit");
    }
}

module.exports = router;
