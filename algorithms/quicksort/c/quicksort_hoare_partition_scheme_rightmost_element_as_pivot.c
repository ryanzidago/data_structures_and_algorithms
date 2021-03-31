#include <stdio.h>

void quicksort(int array[]);
void _quicksort(int array[], int left_index, int right_index);
int partition(int array[], int left_index, int right_index);
void swap(int array[], int i, int j);

int main(void) {
  int array[6] = {0, 5, 2, 6, 1, 3};
  int length = sizeof(array) / sizeof(array[0]);
  
  quicksort(array);

  for (int i = 0; i < length; i++) {
    printf("%d\n", array[i]);
  }
}

void quicksort(int *array) {
  int left_index = 0;

  int length = 0;
  while (array[length] != NULL) {
    length += 1;
  }

  int right_index = length - 1;

  _quicksort(array, left_index, right_index);
}

void _quicksort(int array[], int left_index, int right_index) {
  if (left_index >= right_index) {
    return;
  }
  int pivot_index = partition(array, left_index, right_index);
  _quicksort(array, left_index, pivot_index - 1);
  _quicksort(array, pivot_index + 1, right_index);
}

int partition(int array[], int left_index, int right_index) {
  int pivot_index = right_index;
  right_index -= 1;

  while (1) {
    while (array[left_index] < array[pivot_index]) {
      left_index += 1;
    }

    while (array[right_index] > array[pivot_index]) {
      right_index -= 1;
    }

    if (left_index >= right_index) {
      swap(array, left_index, pivot_index);
      return left_index;
    } else {
      swap(array, left_index, right_index);
      left_index += 1;
    }
  }
}

void swap(int array[], int i, int j) {
    int temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}
