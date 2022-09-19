#!/usr/bin/env python
# coding: utf-8

# In[10]:


import subprocess
import time
import matplotlib.pyplot as plt


# ## Functions for benchmarking

# In[37]:


def run_kmerfinder(file_1, file_2, kmer_size, hashtable_size, use_rolling=False):
    """
    Description:
        This function runs KmerCatcher based on input
    Args:
        1.
        2.
        3.
    """
    args = ["./KmerFinder", f"{file_1}", f"{file_2}", f"{kmer_size}", f"{hashtable_size}"]
    if use_rolling:
        args += ["--use_rolling"]
    print(args)
    subprocess.run(args)


# ## Testing impact of kmer size

# In[50]:


file_1 = "test_files/bacterial_genome_1.txt"
file_2 = "test_files/bacterial_genome_2.txt"
hashtable_size = 50000000
benchmarking = {}
for use_rolling in [1,0]:
    for kmer_size in range(10,500,10):
        start = time.time()
        run_kmerfinder(file_1, file_2, kmer_size, hashtable_size, use_rolling)
        time_to_run = time.time() - start
        benchmarking[f"{use_rolling},{hashtable_size},{kmer_size}"] = time_to_run
        print(f"for k={kmer_size} it took {time_to_run}")


# In[51]:


k_arr = []
time_arr = []
for k, t in benchmarking.items():
    if int(k.split(",")[0]) == 0:
        k_arr.append(int(k.split(",")[-1]))
        time_arr.append(round(t,2))
        
k_arr2 = []
time_arr2 = []
for k, t in benchmarking.items():
    if int(k.split(",")[0]) == 1:
        k_arr2.append(int(k.split(",")[-1]))
        time_arr2.append(round(t,2))


# In[56]:


plt.figure(figsize=(10, 5), dpi=80)
plt.plot(k_arr, time_arr, label="without rolling")
plt.plot(k_arr2, time_arr2, label="with rolling")
plt.title("Benchmarking Rolling vs non-rolling Hash Functions \n'")
plt.xlabel("kmer/pattern Size")
plt.ylabel("Time (seconds)")
plt.legend()


# ## testing impact of hash table size

# In[57]:


file_1 = "test_files/bacterial_genome_1.txt"
file_2 = "test_files/bacterial_genome_2.txt"
hashtable_size = 50000000
kmer_size=20
benchmarking2 = {}
for hashtable_size in range(100000,1000000,100000):
    start = time.time()
    run_kmerfinder(file_1, file_2, kmer_size, hashtable_size, 1)
    time_to_run = time.time() - start
    benchmarking2[f"{use_rolling},{hashtable_size},{kmer_size}"] = time_to_run
    print(f"for k={hashtable_size} it took {time_to_run}")


# In[59]:


for hashtable_size in range(1000000,2000000,100000):
    start = time.time()
    run_kmerfinder(file_1, file_2, kmer_size, hashtable_size, 1)
    time_to_run = time.time() - start
    benchmarking2[f"{use_rolling},{hashtable_size},{kmer_size}"] = time_to_run
    print(f"for k={hashtable_size} it took {time_to_run}")


# In[82]:


# get load factor and time
loadfactor_arr = []
time_arr = []
input_count = len(open(file_1).readlines())
for k, t in benchmarking2.items():
    loadfactor_arr.append(int(k.split(",")[1]))
    time_arr.append(round(t,2))


# In[83]:


# plotting
plt.figure(figsize=(10, 5), dpi=80)
plt.plot(loadfactor_arr, time_arr, label="without rolling")
plt.plot(loadfactor_arr, time_arr, label="with rolling")
plt.title("Impact of Load Factor on Substring Finding \n'")
plt.xlabel("Table Size")
plt.ylabel("Time (seconds)")
plt.legend()


# In[ ]:




