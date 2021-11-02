from flask import Flask, redirect, url_for, request
from flask_cors import CORS, cross_origin
import scipy, json, datetime
from scipy.stats import variation

app = Flask(__name__)
CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

@app.route('/model', methods = ['POST'])
@cross_origin()
def getmodel():
    schedule = request.get_json()
    response_data = []
    monday_sec = datetime.datetime.strptime(schedule['monday'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp()
    for item in schedule['listOfEvents']:
        response_data.append({
            "title": item['title'],
            "start": (datetime.datetime.strptime(item['start'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp() - monday_sec)/3600,
            "end": (datetime.datetime.strptime(item['end'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp() - monday_sec)/3600,
            "source": item['source']
        })
    GetNewSchedule(response_data)
    return json.dumps(response_data)


def GetNewSchedule(schedule):
    listOfFreeTime = []
    lastEndTime = 0
    for i in range(len(schedule)):
        if(i == len(schedule) - 1):
            listOfFreeTime.append(schedule[i]['start'] - lastEndTime - 1)
            listOfFreeTime.append((schedule[i]['end'] - 7 * 24 - 1))
        else:
            listOfFreeTime.append(schedule[i]['start'] - lastEndTime - 1)
            lastEndTime = schedule[i]['end']
    print(scipy.mean(listOfFreeTime))
    print(variation(listOfFreeTime))
    print(listOfFreeTime)
    

# Add derivate function and gradient descent

if __name__ == '__main__':
   app.run(debug = True)