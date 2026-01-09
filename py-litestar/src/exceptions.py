class WeiboException(Exception):
  def __init__(self, msg: str):
    super().__init__(msg)
    self.msg = msg


class LitestarException(WeiboException):
  def __str__(self):
    return f"Litestar Exception: {self.msg}"


class MarshmallowException(WeiboException):
  def __str__(self):
    return f"Marshmallow Exception: {self.msg}"


class NiquestsException(WeiboException):
  def __str__(self):
    return f"Niquests Exception: {self.msg}"


class PiccoloException(WeiboException):
  def __str__(self):
    return f"Piccolo Exception: {self.msg}"
