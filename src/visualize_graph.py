"""グラフの入力は、
        https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A&lang=ja
    の入力と対応
"""

import math
import pprint
import cv2
import numpy as np

HEIGHT = 1000
# HEIGHT = 8000
WIDTH = HEIGHT * 2
INF = 1 << 60
print("INF: ", INF)


def draw_circle(img, yc, xc, r):
    """円を描く"""
    for y in range(-r, r):
        x_max = int(math.sqrt((r**2 - y**2)))
        for x in range(-x_max, x_max):
            img[yc + y][xc + x][:] = 255

def draw_line(img, ys, xs, ye, xe, line_width):
    """直線を描く"""
    dist = max(abs(ye-ys), abs(xe-xs))
    y_list = [int(ys + i * (ye-ys) / dist) for i in range(dist)]
    x_list = [int(xs + i * (xe-xs) / dist) for i in range(dist)]
    if abs(ye-ys) < abs(xe-xs):
        for (y, x) in zip(y_list, x_list):
            for w in range(-line_width, line_width):
                img[y+w][x][:] = 255
    else:
        for (y, x) in zip(y_list, x_list):
            for w in range(-line_width, line_width):
                img[y][x+w][:] = 255

def draw_edge(img, ys, xs, ye, xe, r, line_width, long, theta=(15 / 180 * math.pi)):
    """矢印付きの線を描く"""
    unit_vector = np.array([xs- xe, ys - ye]) / math.sqrt((xs- xe) ** 2 + (ys - ye) ** 2)
    xs = int(xs - unit_vector[0] * r)
    ys = int(ys - unit_vector[1] * r)
    xe = int(xe + unit_vector[0] * r)
    ye = int(ye + unit_vector[1] * r)
    draw_line(img, ys, xs, ye, xe, line_width)
    rotation1 = np.array([[math.cos(theta), -math.sin(theta)], 
                            [math.sin(theta), math.cos(theta)]])
    rotation2 = np.array([[math.cos(-theta), -math.sin(-theta)], 
                            [math.sin(-theta), math.cos(-theta)]])
    vector1 = rotation1 @ unit_vector * long
    vector2 = rotation2 @ unit_vector * long
    # 矢印の先端用の直線を描く
    draw_line(img, ye, xe, int(ye + vector1[1]), int(xe + vector1[0]), line_width)
    draw_line(img, ye, xe, int(ye + vector2[1]), int(xe + vector2[0]), line_width)


# モードの選択
# print("Please specify the mode number. 0: Without weight, 1: With weight")
# mode = int(input())
mode = 0

# mode == 0: 重みなし
# mode == 1: 重み込み
# 1行目: v_num, e_num, r
# v_numはGの頂点の数, e_numはGの頂点の辺の数、rは始点の番号
# 2行目以降: s, t, d
# sは矢印の根本の頂点、tは矢印の先端の頂点、dは重み

# 1行目の入力
line0 = int(input())
# line0 = map(int, input().split())
if mode == 0:
    # v_num, e_num = line0
    v_num = line0
    e_num = v_num - 1 # エッジの数
    # e_num = v_num
    r = 0
else:
    v_num, e_num, r = line0
# v_numはGの頂点の数, e_numはGの頂点の辺の数、rは始点の番号

# 2行目以降の入力
graph = [[] for i in range(v_num)]
for i in range(e_num):
    line = map(int, input().split())
    if mode == 0:
        s, t = line
        s -= 1
        t -= 1
        d = 0
    else:
        s, t, d = line
    graph[s].append({"neighbor": t, "weight": d})

print("graph")
pprint.pprint(graph)

seen = [False for i in range(v_num)]
distance = [INF for i in range(v_num)]
distance[r] = 0

# # トポロジカルソートする
# topological_sorted_list = []
# def dfs(v):
#     seen[v] = True
#     for e in graph[v]:
#         if seen[e["neighbor"]]:
#             continue
#         dfs(e["neighbor"])
#     topological_sorted_list.append(v)
# dfs(r)
# topological_sorted_list.reverse()
# print("topological_sorted_list: ", topological_sorted_list)

# 幅優先探索で最短経路を求める
todo = [r]
while len(todo) != 0:
    v = todo.pop(0)
    print("v: ", v)
    for e in graph[v]:
        print("v: ", v, "e: ", e["neighbor"])
        # print("e", e["neighbor"], "v", v, "distance[v]", distance[v])
        if distance[e["neighbor"]] != INF:
            continue
        distance[e["neighbor"]] = distance[v] + 1
        print('distance[e["neighbor"]]: ', distance[e["neighbor"]])
        todo.append(e["neighbor"])

print("distance", distance)

# v * v のマスを用意するべき
# 例: 1000 * 2000
distance_hist = [0 for i in range(v_num)]
img = np.zeros((HEIGHT, WIDTH, 3))

# 始点からたどり着けない頂点の距離を、辿り着ける最大距離+1に変更する(2stepで実行)
# (1)始点からたどり着けない頂点の距離を、辿り着ける-1に変更する
for i, d in enumerate(distance):
    if d == INF:
        print("INF")
        distance[i] = -1
        print("distance[i]", distance[i])
distance_max = max(distance)
# (2)始点からたどり着けない頂点の距離を、辿り着ける最大距離+1に変更する
for i, d in enumerate(distance):
    if d == -1:
        distance[i] = distance_max + 1
print("distance", distance)


v_yx = []
r=int(min(HEIGHT, WIDTH) / v_num / 4)
# r=int(min(HEIGHT, WIDTH) / distance_max / 4) # 円がデカすぎて重なる
for v in range(v_num):
    # 頂点の円の描画
    distance_hist[distance[v]] += 1
    # h = int(HEIGHT * (distance_hist[distance[v]] + 1) / (v_num + 1)) # 同じ高さの頂点が3つ以上あると、左端と右端の頂点を繋いだとき、間にいる頂点が貫かれてしまう
    h = int(HEIGHT * (v + 1) / (v_num + 1)) # 左上から右下に流れる

    # w = int(WIDTH * (distance[v] + 1) / (distance_max + 2))
    w = int(WIDTH * (distance[v] + 1) / (v_num + 1))

    v_yx.append([h, w])
    draw_circle(img, h, w, r)

    # 頂点の番号の描画
    # font_pixel_size = min(HEIGHT, WIDTH) / v_num / 2 / 2 
    font_pixel_size = r
    
    fontscale = font_pixel_size / 20 # 25.0で500ピクセル。1/20の縮尺
    v_digit = len(str(v))
    print("v_digit: ", v_digit)
    cv2.putText(img,
                text="{:>1}".format(v), # 1桁のとき
                # text="{:>2}".format(v), # 2桁のとき
                org=(w - int(font_pixel_size / 2 * v_digit), h + int(font_pixel_size / 2)), # 1桁のとき
                # org=(w - int(font_pixel_size), h + int(font_pixel_size / 2)), # 2桁のとき
                fontFace=cv2.FONT_HERSHEY_SIMPLEX,
                fontScale=fontscale,
                color=(0, 0, 255),
                thickness=2,
                lineType=cv2.LINE_4)

for v in range(v_num):
    # エッジの描画
    h = v_yx[v][0]
    w = v_yx[v][1]
    for edge in graph[v]:
        v_next = edge["neighbor"]
        h_next = v_yx[v_next][0]
        w_next = v_yx[v_next][1]
        print("v, v_next, h, w, h_next, w_next: ", v, v_next, h, w, h_next, w_next)
        # draw_line(img, 100, 0, 100, 1000, line_width=10)
        draw_edge(img, h, w, h_next, w_next, r, line_width=max(1, int(r / 10)), long=r)
        # 1個ずつエッジを描画する
        # cv2.imwrite("graph.png", img)
        # input()


cv2.imwrite("graph.png", img)


# cv2.putText(img,
#             text="sample",
#             org=(100, 300),
#             fontFace=cv2.FONT_HERSHEY_SIMPLEX,
#             fontScale=1.0,
#             color=(0, 255, 0),
#             thickness=2,
#             lineType=cv2.LINE_4)
