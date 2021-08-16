from typing import TypeVar, Generic, List
import heapq

T = TypeVar('T')

class PriorityQueue(Generic[T]):
    def __init__self(self) -> None:
        self._container: List[T] = []

    @property 
    def empty(self) -> bool:
        return not self._container

    def push(self, item: T) -> None:
        heapq.heappush(self._container, item)
    
    def pop(self) -> T:
        return heapq.heappop(self._container)

    def __repr__(self) -> str:
        return repr(self._container)

    