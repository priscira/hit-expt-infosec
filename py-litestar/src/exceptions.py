class WeiboException(Exception):
  def __init__(self, msg: str):
    super().__init__(msg)
    self.msg = msg


class WeiboLitestarException(WeiboException):
  def __str__(self):
    return f"Litestar Exception: {self.msg}"


class WeiboMarshmallowException(WeiboException):
  def __str__(self):
    return f"Marshmallow Exception: {self.msg}"


class WeiboNiquestsException(WeiboException):
  def __str__(self):
    return f"Niquests Exception: {self.msg}"


class WeiboPiccoloException(WeiboException):
  def __str__(self):
    return f"Piccolo Exception: {self.msg}"
