from typing import TypeVar, Generic, List

T = TypeVar('T')

class Stack(Generic[T]):
    def __init__(self) -> None:
        self._container: List[T] = []

    @property 
    def empty(self) -> bool:
        return not self._container

    def push(self, item: T) -> None:
        self._container.pop()

    def pop(self) -> T:
        return self._container.pop()

    def __repr__(self) -> str:
        return repr(self._container)