from datem.internal import expr


def __getattr__(name):
    return getattr(expr, name)
