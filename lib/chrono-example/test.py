# -*- coding: utf-8 -*-
# @Author: jankincai
# @Date:   2023-12-06 13:48:48
# @Last Modified by:   jankincai
# @Last Modified time: 2023-12-06 13:50:06
import datetime  
  
timestamp = 132524746251044830  
timestamp_ms = int(timestamp) / 1000  # 将时间戳转换为毫秒数  
datetime_obj = datetime.datetime.utcfromtimestamp(timestamp_ms)  
  
print(datetime_obj)
# 2022-08-25 14:30:48.830000