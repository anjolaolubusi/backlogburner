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
	  res.statusCode = 200;
    console.log(req.body);
    var response_data = [];
    var monday_sec = new Date(req.body.monday).getTime();
    for(let i = 0; i < req.body.listOfEvents.length; i++){
      response_data.push(
        {
          title: req.body.listOfEvents[i].title,
          start: (new Date(req.body.listOfEvents[i].start).getTime() - monday_sec)/3600000,
          end: (new Date(req.body.listOfEvents[i].end).getTime() - monday_sec)/3600000,
          source: req.body.listOfEvents[i].source
        }
      )
    }
    console.log(response_data)
	  res.send(response_data);
  })
