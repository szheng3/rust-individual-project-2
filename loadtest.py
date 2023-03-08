from locust import HttpUser, task, between


class MyUser(HttpUser):
    wait_time = between(5, 10)

    @task
    def post_request(self):
        payload = {
            "context": "scikit-learn is a free software machine learning library for the Python programming language. It features various classification, regression and clustering algorithms including support-vector m",
            "minlength": 10,
            "model": "Bart"
        }
        headers = {
            "Content-Type": "application/json"
        }
        self.client.post("/api/summary", json=payload, headers=headers)
