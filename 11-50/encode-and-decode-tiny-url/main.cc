#include <math.h>

#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    Solution() {
        srand(time(NULL));
    }

    string encode(string longUrl) {
        static string prefix = "https://www.qixin.com/";
        if (long2Short_.count(longUrl)) {
            return prefix + long2Short_[longUrl];
        }
        string shortUrl = dict_.substr(0, len_);
        while (short2Long_.count(shortUrl)) {
            std::random_shuffle(dict_.begin(), dict_.end());
            shortUrl = dict_.substr(0, len_);
        }
        long2Short_[longUrl] = shortUrl;
        short2Long_[shortUrl] = longUrl;
        return prefix + shortUrl;
    }

    string decode(string shortUrl) {
        auto pos = shortUrl.find_last_of('/');
        auto suffix = shortUrl.substr(pos + 1);
        if (short2Long_.count(suffix)) {
            return short2Long_[suffix];
        } else {
            return shortUrl;
        }
    }

private:
    unordered_map<string, string> long2Short_, short2Long_;
    int len_ = 7;
    string dict_ = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
};

int main(int argc, char ** argv)
{
    Solution solution;
    string shortUrl = solution.encode("https://www.baidu.com");
    string longUrl = solution.decode(shortUrl);
    cout << "shortUrl = " << shortUrl << ", longUrl = " << longUrl << endl;
    shortUrl = solution.encode("https://www.google.com");
    longUrl = solution.decode(shortUrl);
    cout << "shortUrl = " << shortUrl << ", longUrl = " << longUrl << endl;
    shortUrl = solution.encode("https://www.tita.com");
    longUrl = solution.decode(shortUrl);
    cout << "shortUrl = " << shortUrl << ", longUrl = " << longUrl << endl;
    return 0;
}
