var express = require('express')
//var mysql = require('mysql')
var app = express();

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

  async function GetNewSchedule(){
	  return "New Schedule"
  }
  
  app.post('/model', function(req, res, next){
	res.statuscode = 200;
	res.send("MATH STUFF");
  })

  app.post('/model2', function(req, res, next){
	  res.statusCode = 200;
	  res.send(req.body);
  })
