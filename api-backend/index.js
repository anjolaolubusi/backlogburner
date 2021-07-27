var express = require('express')
var mysql = require('mysql')
var app = express();

var config = {
	host: "localhost",
	user: "client",
	password: "password",
	database: "ProjTest"
}

class Database {
    constructor( config ) {
        this.connection = mysql.createConnection( config );
    }
    query( sql, args ) {
        return new Promise( ( resolve, reject ) => {
            this.connection.query( sql, args, ( err, rows ) => {
                if ( err )
                    return reject( err );
                resolve( rows );
            } );
        } );
    }
    close() {
        return new Promise( ( resolve, reject ) => {
            this.connection.end( err => {
                if ( err )
                    return reject( err );
                resolve();
            } );
        } );
    }
}

myDatabase = new Database(config)
app.use(function (req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
  res.header("Access-Control-Allow-Methods", "*");
  next();
});

app.use(express.urlencoded({ extended: true}));
app.use(express.json());

const portNumber = 3000;
app.listen(portNumber, () => {
	console.log("Listening on localhost:" + portNumber);
});
const statusOk = 200;


//global functions

//Media
app.get('/api/media', function(req, res, next){
	myDatabase.query("SELECT ID FROM Users WHERE username = \"" + req.query.name + "\"")
		.then( rows => {
			if(rows.length == 0){
				throw 'EMPTY ARR'
			}
			UserID = rows[0].ID;
			return myDatabase.query("SELECT * FROM Media WHERE UserID=" + UserID);
		})
		.then(rows => {
			res.statuscode = 200;
			res.send(rows);
		})
		.catch( (err) => {
			next(err);
		});
});


app.delete('/api/media', function(req, res, next){
	myDatabase.query("DELETE FROM Media WHERE ID = " + req.query.id)
		.catch( (err) => {
			next(err);
		})
})

app.post('/api/remind', function(req, res, next){
	myDatabase.query('UPDATE Media SET remind= NOT remind WHERE id = ' + req.body.id)
	.catch( (err) => {
			next(err);
		})
})

app.post('/api/media', function(req, res, next){
	myDatabase.query(`INSERT INTO Media(UserID, MediaTypeID, MediaName, Remind, StartDate, EndDate) VALUES (${req.body.userID}, ${req.body.mediaTypeId}, \"${req.body.mediaName}\", ${req.body.reminder}, \"${req.body.startDate}\", \"${req.body.endDate}\")`)
	.then(
		myDatabase.query(`SELECT * FROM Media WHERE UserID=${req.body.userID} AND MediaTypeID = ${req.body.mediaTypeId} AND MediaName = \"${req.body.mediaName}\"`)
	)
	.then(
		(rows) => {
			res.statusCode = 200;
			res.send(rows)
		}
	)
	.catch( (err) => {
		next(err);
	})
})


//Users
app.get('/api/users', function(req, res, next){
	myDatabase.query("SELECT * FROM Users WHERE username = \"" + req.query.name + "\"")
		.then(rows => {
			res.statuscode = 200;
			res.send(rows);
		})
		.catch( (err) => {
			next(err);
		});
});


//Mediatypes
app.get('/api/mediatype', function(req, res, next){
	myDatabase.query("SELECT * FROM MediaTypes;")
		.then(rows => {
			res.statusCode = 200;
			res.send(rows)
		})
		.catch( (err) => {
			next(err);
		})
})
