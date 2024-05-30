#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int main() {
  char num_of_nums_str[1024];
  fgets(num_of_nums_str, 1024, stdin);
  int num_of_nums = atoi(num_of_nums_str);

  char nums_str[9 * 100000];
  fgets(nums_str, 9 * 100000, stdin);

  int nums[num_of_nums];
  int j = 0;

  int i = 0;
  char c = nums_str[i];
  char cur_num[8] = "";
  int cur_len = 0;
  while (1) {
    if ((c == '\0' || c == ' ' || c == '\n') && cur_len > 0) {
      cur_num[cur_len] = '\0';
      nums[j++] = atoi(cur_num);
      cur_num[0] = '\0';
      cur_len = 0;
    } else {
      cur_num[cur_len++] = c;
    }

    if (c == '\0') break;
    c = nums_str[++i];
  }

  int min = INT_MAX;
  for (int i = 0; i < num_of_nums; i++) {
    int n = abs(nums[i]);
    if (n < min) {
      min = abs(n);
    }
  }

  printf("%d\n", min);
}
