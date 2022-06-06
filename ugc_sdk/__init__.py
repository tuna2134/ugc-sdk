from ugc_sdk_core import UgcGateway


class UgcClient:
    gateway = UgcGateway()

    def connect(self):
        self.gateway.connect()