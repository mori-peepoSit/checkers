import pyperclip
import itertools as it
from time import perf_counter_ns
from functools import wraps
from sys import setrecursionlimit
import struct
import math
import numpy as np


def read(path, splitter="\n"):
    '''useless read function'''
    with open(path, "r") as file:
        return file.read().split(splitter)


def grid(array):
    '''iterates over 2-d array in all 4 directions'''
    R = len(array) # row index
    C = len(array[0]) # columns index
    output1,output2,output3,output4 = "rc\n", "rc\n", "rc\n", "rc\n"
    for r in range(R): # rowwise
        for c in range(C):
            output1 += str(r) + str(c) + " " # 🡆 + 🡇
            output2 += str(r) + str(C-c-1) + " "# 🡄 + 🡇
        output1 += "\n"; output2 += "\n"
    for c in range(C): # colwise
        for r in range(R):
            output3 += str(r) + str(c) + " " # 🡇 + 🡆
            output4 += str(R-1-r) + str(c) + " " # 🡅 + 🡆
        output3 += "\n"; output4 += "\n"
    return output1, output2, output3, output4


def square_multiply(base, exp, mod):
    '''bit-manipulation method to calculate base^exp % mod quickly'''
    b = 1
    A = base % mod
    if exp & 1:
        b = base % mod
    exp >>= 1
    while exp > 0:
        A = (A * A) % mod
        if exp & 1:
            b = (A * b) % mod
        exp >>= 1
    return b % mod
#modulo = square_multiply(5, 100000, 1000000007)


def spam(string, emote='PokPikachu'):
    '''twitch spam'''
    output = emote+" "; 
    for i in string:
        output += i + " " + emote + " "  
    pyperclip.copy(output)

def bobo_spam(message, n=3):
    output = ""
    for i in message.split(' '):
        for _ in range(n):
            output += i + ' '
    pyperclip.copy(output)
# spam('answer xini grrr')

def runtime(function, *args):
    '''returns result, runtime of function in ns / 10**9 = secs'''
    starttime = perf_counter_ns()
    result = function(*args)
    dur = perf_counter_ns() - starttime
    return result, dur/(10**9)


def range_included(start1, end1, start2, end2):
    '''range overlap / include'''
    if start1<=start2 and end1 >= end2:
        return True
    return False


def range_overlap(start1, end1, start2, end2):
    if start1<=end2 and end1 >= end2:
        return True
    elif start2<=end1 and end2 >= end1:
        return True
    return False


def time_file_convert():
    with open('F:\\assignments\\C_Stuff\\.vscode\\study.txt', 'r') as a:
        lines = a.readlines()
        dict0 = {}
        for i in lines:
            duration = i.split(' ')[0].split(':')
            if len(duration)==3:
                duration_fl = int(duration[0])*3600+int(duration[1])*60+int(duration[2])
            else:
                duration_fl = float(duration[0])
            date = i.split(' ')[2].removesuffix('\n')
            if dict0.get(date) == None:
                dict0.update({date: duration_fl})
            else:
                dict0[date] += duration_fl
        print(dict0)
        for x in dict0:
            print(F"{dict0[x]} secs_on {x}\n")


def memoize(func): 
    '''
    # simple cache
    # setrecursionlimit(10000)
    @memoize
    def fibonacci(n):
        if n < 2:
            return n
        return fibonacci(n-1) + fibonacci(n-2)
    '''
    cache = {}
    @wraps(func)
    def wrapper(*args, **kwargs):
        key = str(args) + str(kwargs)
        if key not in cache:
            cache[key] = func(*args, **kwargs)
        return cache[key]
    return wrapper


if __name__ == 'main':
    # miscellaneous:
    data = list(range(10)); i = 5; buffer_size = 2
    set(data[i-buffer_size+1:i+1])
    list(it.combinations(range(5), 2))
    list(it.combinations_with_replacement(range(5), 2))
    a = np.array([[2, 4], [3, 3]]); b = np.array([[5, 1], [2, 4]])
    # from pandas import value_counts
    print(int('2F',16))
    a @ b # matrix multiplication
    math.radians(180)


def struct_isqrt(number):
    threehalfs = 1.5
    x2 = number * 0.5
    y = number
    
    packed_y = struct.pack('f', y)       
    i = struct.unpack('i', packed_y)[0]  # treat float's bytes as int 
    i = 0x5f3759df - (i >> 1)            # arithmetic with magic number
    packed_i = struct.pack('i', i)
    y = struct.unpack('f', packed_i)[0]  # treat int's bytes as float
    
    y = y * (threehalfs - (x2 * y * y))  # Newton's method
    return y