from flask import Flask, redirect, url_for, request
from flask_cors import CORS, cross_origin
import json, datetime, numpy as np
from scipy.stats import variation
from scipy.optimize import minimize
from scipy.optimize import Bounds

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

app = Flask(__name__)
CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

newEvent_start = 0
response_data = []
event_length = 0

@app.route('/model', methods = ['POST'])
@cross_origin()
def getmodel():
    global newEvent_start
    global response_data
    global event_length
    schedule = request.get_json()
    print(schedule)
    monday_sec = datetime.datetime.strptime(schedule['monday'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp()
    response_data = []
    for item in schedule['listOfEvents']:
        response_data.append({
            "title": item['title'],
            "start": (datetime.datetime.strptime(item['start'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp() - monday_sec)/3600,
            "end": (datetime.datetime.strptime(item['end'], "%Y-%m-%dT%H:%M:%S.000Z").timestamp() - monday_sec)/3600,
            "source": item['source']
        })
    event_length = int(schedule['newEvent']['length'])
    response_data.append({
        "title": schedule['newEvent']['title'],
        "start": newEvent_start,
        "end": newEvent_start + int(schedule['newEvent']['length']),
        "source": "M"
    })
    bounds = Bounds([0], [168 - event_length])
    s0 = 0
    newTest = minimize(GetFreeTimeList, s0, method='trust-constr', bounds=bounds)
    return json.dumps(response_data, cls=NumpyEncoder)


def GetFreeTimeList(start_point):
    global response_data
    global event_length
    response_data[-1]['start'] = start_point
    response_data[-1]['end'] =  start_point + event_length
    response_data.sort(key=lambda x: x['start'], reverse=False)
    listOfFreeTime = np.zeros(len(response_data))
    lastEndTime = 0
    for i in range(len(response_data)):
        if(i == len(response_data) - 1):
            listOfFreeTime[i] = (response_data[i]['start'] - lastEndTime - 1)
            listOfFreeTime[i] = ((7 * 24 - 1) - (response_data[i]['end']))
        else:
            listOfFreeTime[i] = (response_data[i]['start'] - lastEndTime - 1)
            lastEndTime = response_data[i]['end'] + 1
    return variation(listOfFreeTime)
    
if __name__ == '__main__':
   app.run(debug = True)