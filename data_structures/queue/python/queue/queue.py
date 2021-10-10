from typing import TypeVar, Generic, Deque

T = TypeVar('T')

class Queue(Generic[T]):
    def __init__(self) -> None:
        self._container: Deque[T] = Deque()

    @property 
    def empty(self) -> bool: 
        return not self._container 

    def push(self, item: T) -> None:
        self._container.append(item)

    def pop(self) -> T:
        return self._container.popleft()

    def __repr__(self) -> str: 
        return repr(self._container)
