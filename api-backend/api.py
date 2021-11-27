from flask import Flask, redirect, url_for, request
from flask_cors import CORS, cross_origin
import json, datetime, numpy as np
from scipy.stats import variation
from scipy.optimize import minimize
from scipy.optimize import Bounds
from scipy.optimize import NonlinearConstraint

class NumpyEncoder(json.JSONEncoder):
    """ Special json encoder for numpy types """
    def default(self, obj):
        if isinstance(obj, (np.int_, np.intc, np.intp, np.int8,
                            np.int16, np.int32, np.int64, np.uint8,
                            np.uint16, np.uint32, np.uint64)):
            return int(obj)
        elif isinstance(obj, (np.float_, np.float16, np.float32,
                              np.float64)):
            return float(obj)
        elif isinstance(obj, (np.ndarray,)):
            return obj.tolist()
        return json.JSONEncoder.default(self, obj)

# Change code so that repsonse_data does not have new event.

app = Flask(__name__)
CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

newEvent_start = 0
response_data = []
event_length = 0
newEvent_title = ""

def getNewEventIndex(new_title):
    global response_data
    newEvent_index = -1
    for i in range(len(response_data)):
        if(response_data[i]['title'] == new_title):
            newEvent_index = i
    return newEvent_index

def checkScheduleViolation(x):
    global newEvent_title
    global response_data
    violation = 0
    E_i = getNewEventIndex(newEvent_title)
    for i in range(len(response_data)):
        if(i != E_i):
            print("{} {} - {}".format(response_data[i]['title'], response_data[i]['start'], response_data[i]['end']))
            if(response_data[i]['start'] <= response_data[E_i]['end'] <= response_data[i]['end'] or response_data[i]['start'] <= response_data[E_i]['start'] <= response_data[i]['end']):
                violation += 1
    print("Num of violations is: ", violation)
    return violation


"""
Replace model with ACO (Or combinatoric model) where the quality function is: maximazing the smallest break possible
"""

@app.route('/model', methods = ['POST'])
@cross_origin()
def getmodel():
    global newEvent_start
    global response_data
    global event_length
    global newEvent_title

    schedule = request.get_json()
    monday_sec = datetime.datetime.strptime(schedule['monday'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp()
    response_data = []
    for item in schedule['listOfEvents']:
        response_data.append({
            "title": item['title'],
            "start": (datetime.datetime.strptime(item['start'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp() - monday_sec)/3600,
            "end": (datetime.datetime.strptime(item['end'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp() - monday_sec)/3600,
            "source": item['source']
        })
    newEvent_title = schedule['newEvent']['title']
    event_length = int(schedule['newEvent']['length'])
    response_data.append({
        "title": schedule['newEvent']['title'],
        "start": newEvent_start,
        "end": newEvent_start + int(schedule['newEvent']['length']),
        "source": "M"
    })
    bounds = Bounds([0], [168 - event_length])
    viol_const = NonlinearConstraint(checkScheduleViolation, -np.inf, 0, hess=lambda x,y: 0)
    newTest = minimize(GetFreeTimeList, newEvent_start, method='trust-constr', bounds=bounds, constraints=viol_const, options={'verbose': 1})
    newEvent_index = getNewEventIndex(newEvent_title)
    newSchedule = response_data[newEvent_index]
    newSchedule["start"] = datetime.datetime.fromtimestamp(int(newSchedule["start"] * 3600 + monday_sec)).isoformat()
    newSchedule["end"] = datetime.datetime.fromtimestamp(int(newSchedule["end"] * 3600 + monday_sec)).isoformat()

    return json.dumps(newSchedule, cls=NumpyEncoder)


def GetFreeTimeList(start_point):
    global response_data
    global event_length
    global newEvent_title
    response_data[getNewEventIndex(newEvent_title)]['start'] = start_point
    response_data[getNewEventIndex(newEvent_title)]['end'] =  start_point + event_length
    response_data.sort(key=lambda x: x['start'], reverse=False)
    listOfFreeTime = np.zeros(len(response_data))
    lastEndTime = 0
    E_i = getNewEventIndex(newEvent_title)
    print("New Event: {} {} - {}".format(response_data[E_i]['title'], response_data[getNewEventIndex(newEvent_title)]['start'], response_data[getNewEventIndex(newEvent_title)]['end']))
    for i in range(len(response_data)):
        if(i == len(response_data) - 2):
            listOfFreeTime[i] = (response_data[i]['start'] - lastEndTime - 1)
            listOfFreeTime[i+1] = ((7 * 24 - 1) - (response_data[i]['end']))
        else:
            listOfFreeTime[i] = (response_data[i]['start'] - lastEndTime - 1)
            lastEndTime = response_data[i]['end'] + 1
    return variation(listOfFreeTime)

    
if __name__ == '__main__':
   app.run(debug = True)