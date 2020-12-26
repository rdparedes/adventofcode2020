#include <iostream>
#include <vector>
#include <fstream>
#include <string>

using namespace std;

/* Given a list of numbers, find 3 numbers that sum to 2020. Then multiply them.
*/

vector<int> ReadFile(const string &filename)
{
    vector<int> contents = {};
    fstream input_file;
    input_file.open(filename, ios::in);
    if (!input_file)
    {
        cout << "Error opening file!";
        return contents;
    }
    string buffer;
    while (!input_file.eof())
    {
        getline(input_file, buffer, '\n');
        try
        {
            int current_number = stoi(buffer);
            contents.push_back(current_number);
        }
        catch (exception)
        {
        }
    }
    return contents;
}

int main()
{
    vector<int> data = ReadFile("src/input.txt");

    auto data_length = data.size();
    for (int i = 0; i < data_length; ++i)
    {
        for (int j = 0; j < data_length; ++j)
        {
            int i_val = data[i];
            int j_val = data[j];
            int sum_of_two = i_val + j_val;
            if (i == j || sum_of_two > 2020)
            {
                continue;
            }
            for (int k = 0; k < data_length; ++k)
            {
                if (k == j || k == i)
                {
                    continue;
                }
                int k_val = data[k];
                int sum_of_three = sum_of_two + k_val;
                if (sum_of_three == 2020)
                {
                    long result = i_val * j_val * k_val;
                    cout << "Found one match!: " << i_val << ", " << j_val << ", " << k_val << endl;
                    cout << "Found one match!: " << result << endl;
                    return 0;
                }
            }
        }
    }
}
