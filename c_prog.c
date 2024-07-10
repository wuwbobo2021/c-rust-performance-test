#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
#include <time.h>

static inline struct timespec get_ts() {
	struct timespec ts;
	timespec_get(&ts, TIME_UTC);
	return ts;
}
static inline float ts_diff(struct timespec ts_end, struct timespec ts_beg) {
	float ts = difftime(ts_end.tv_sec, ts_beg.tv_sec);
	ts += (ts_end.tv_nsec - ts_beg.tv_nsec) / 1000.0 / 1000.0 / 1000.0;
	return ts;
}

int main()
{
	int max = 100*1000*1000, sqrt_max;
	//scanf("%d", &max);
	//if (max < 2) return 0;
	
	struct timespec t_start = get_ts();
	
	sqrt_max = sqrt(max); //int
	
	bool* sieve = malloc(max * sizeof(bool));
	memset(sieve, false, max * sizeof(bool));
	
	int* result = malloc(max * sizeof(int));
	int cnt_result = 0;
	
	for (int i = 2; i <= sqrt_max; i++) {
		if (sieve[i - 1]) continue;
		
		result[cnt_result++] = i;
		for (int j = i + i - 1; j < max; j += i)
			sieve[j] = true;
	}
	
	for (int i = sqrt_max + 1; i <= max; i++)
		if (! sieve[i - 1]) result[cnt_result++] = i;
	
	struct timespec t_end = get_ts();

	printf("c_prog: %f s\n", ts_diff(t_end, t_start));
	for (int i = 0; i < 10; i++)
		printf("%d ", result[i]);
	for (int i = cnt_result - 10; i < cnt_result; i++)
		printf("%d ", result[i]);
	printf("\n");
	
	free(sieve); free(result);
	return 0;
}
