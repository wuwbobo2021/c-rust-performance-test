#include <iostream>
#include <vector>
#include <cmath>
#include <ctime>

using namespace std;

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
	//cin >> max;
	//if (max < 2) return 0;

	timespec t_start = get_ts();
	
	sqrt_max = sqrt(max); //int
	
	vector<char> sieve(max, false);
	vector<int> result; result.reserve(max);
	
	for (int i = 2; i <= sqrt_max; i++) {
		if (sieve[i - 1]) continue;
		
		result.push_back(i);
		for (int j = i + i - 1; j < max; j += i)
			sieve[j] = true;
	}
	
	for (int i = sqrt_max + 1; i <= max; i++)
		if (! sieve[i - 1]) result.push_back(i);

	timespec t_end = get_ts();

	cout << "cpp_vect: " << ts_diff(t_end, t_start) << " s" << endl;
	for (int i = 0; i < 10; i++)
		cout << result[i] << ' ';
	for (int i = result.size() - 10; i < result.size(); i++)
		cout << result[i] << ' ';
	cout << endl;
	
	return 0;
}
