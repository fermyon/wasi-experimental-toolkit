from flask import Flask, request

from cloudevents.http import CloudEvent
from host_py import run

app = Flask(__name__)


# create an endpoint at http://localhost:/3000/
@app.route("/<path:path>", methods=["POST"])
def home(path):
    # create a CloudEvent
    attributes = {
        "type": "com.microsoft.steelthread.wasm",
        "source": f"http://127.0.0.1:3030/{path}",
    }
    event = CloudEvent(attributes, request.data)

    # you can access cloudevent fields as seen below
    print(
        f"Found {event['id']} from {event['source']} with type "
        f"{event['type']} and specversion {event['specversion']}"
    )
    run(event)

    return "", 204


if __name__ == "__main__":
    app.run(port=3030)
