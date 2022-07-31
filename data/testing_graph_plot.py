import numpy as np
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches


plt.ion()


def plot_graph(s_lst, v_lst, e_lst, filename=None, radius=0.3):
    xmax, ymax = calc_x_y_max(v_lst)

    plt.figure(figsize=(xmax, ymax))
    ax = plt.gca()

    for s, (x, y) in zip(s_lst, v_lst):
        rect = mpatches.Circle((x, y), radius, ec="#33a02c", fc="#b2df8a", lw=2)

        ax.add_patch(rect)
        plt.text(x, y, s, ha="center", va="center", family='Monaco', size=24)

    for e in e_lst:
        if len(e) == 3:
            u, v, w = e
        else:
            u, v = e
            w = None

        x1, y1 = v_lst[u]
        x2, y2 = v_lst[v]
        dx = x2 - x1
        dy = y2 - y1
        if x1 == x2:
            p = 1 if y2 > y1 else -1
            y1 += radius * p
            dy -= 2 * radius * p
        elif y1 == y2:
            p = 1 if x2 > x1 else -1
            x1 += radius * [-1, 1][int(x2 > x1)]
            dx -= 2 * radius * p
        else:
            dz = np.sqrt((y2 - y1) ** 2 + (x2 - x1) ** 2)
            py = radius * (y2 - y1) / dz # padding
            px = radius * (x2 - x1) / dz
            x1 += px
            y1 += py
            dx -= 2 * px
            dy -= 2 * py

        if w:
            plt.text(x1 + dx / 2, y1 + dy / 2, w, ha="center", va="center",
                     family='Monaco', size=16, backgroundcolor="white")

        arrow = mpatches.FancyArrow(x1, y1, dx, dy, width=0.02,
                                    head_width=0.1, head_length=0.09,
                                    fc="#131926", length_includes_head=True)
        ax.add_patch(arrow)

    plt.axis('equal')
    plt.axis('off')
    plt.tight_layout()

    if filename:
        plt.savefig(filename)
        plt.close()


def calc_x_y_max(v_lst):
    xmax, ymax = 0, 0
    for x, y in v_lst:
        if x > xmax:
            xmax = x
        if y > ymax:
            ymax = y
    xmax += 1
    ymax += 1
    return xmax, ymax


def plot_scc(filename="scc.png"):
    count = 9
    a, b, c, d, e, f, g, h, i = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    v_lst = [(1, 2),
             (2, 2),
             (1, 1),
             (2, 1),
             (3, 3),
             (3.5, 3-np.sqrt(3)/2),
             (4, 3),
             (5, 2),
             (5, 1)]

    e_lst = [
        (a, b),
        (a, c),
        (b, d),
        (b, e),
        (b, i),
        (c, d),
        (d, a),
        (d, h),
        (e, f),
        (f, g),
        (g, e),
        (g, h),
        (h, i),
        (i, h),
    ]

    plot_graph(s_lst, v_lst, e_lst, filename)


def plot_mst(filename="mst.png"):
    count = 9
    a, b, c, d, e, f, g, h, i = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    v_lst = [(1, 2),
             (2, 3),
             (4, 3),
             (6, 3),
             (7, 2),
             (6, 1),
             (4, 1),
             (2, 1),
             (3, 2)]

    e_lst = [
        (a, b, 4),
        (a, h, 8),
        (b, c, 8),
        (b, h, 11),
        (c, d, 7),
        (c, f, 4),
        (c, i, 2),
	(d, e, 9),
	(d, f, 14),
        (e, f, 10),
        (f, g, 2),
        (g, h, 1),
        (g, i, 6),
        (h, i, 7),
    ]

    plot_graph(s_lst, v_lst, e_lst, filename)


# shortest path using a start algorithm
def plot_spa(filename="spa.png", radius=0.4):
    count = 7
    s, a, b, c, d, e, t = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    dx = np.sqrt(3)
    v_lst = [
        (1, 4),
        (1 + dx, 5),
        (1 + dx, 3),
        (1 + 2 * dx, 5),
        (1, 2),
        (1 + dx, 1), #e
        (1 + 3 * dx, 3),
        ]

    #print(np.sqrt(1 + (ymax - 2.5 - np.sqrt(3) / 2) ** 2))
    print(v_lst)
    e_lst = [(s, a, 3),
             (s, d, 2),
             (a, b, 2),
             (b, c, 3),
             (c, t, 3),
             (d, e, 4),
             (e, t, 4.5)]

    plot_graph(s_lst, v_lst, e_lst, filename, radius)


def plot_spn(filename="spn.png"):
    count = 5
    v1, v2, v3, v4, v5 = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    dx = 2 * np.cos(72*np.pi/180) # 边长2
    dy = 2 * np.sin(72*np.pi/180)
    v_lst = [(1, 1 + dy),
             (2 + dx, 1 + dy + 2 * np.sin(36*np.pi/180)),
             (3 + 2 * dx, 1 + dy),
             (3 + dx, 1),
             (1 + dx, 1),
    ]
    e_lst = [
        (v1, v2, 3),
	(v1, v3, 8),
	(v1, v5, -4),
	(v2, v4, 1),
	(v2, v5, 7),
	(v3, v2, 4),
	(v4, v1, 2),
	(v4, v3, -5),
	(v5, v4, 6)
        ]

    plot_graph(s_lst, v_lst, e_lst, filename)

def plot_mbm(filename="mbm.png"):
    count = 12
    x1, x2, x3, x4, x5, x6, y1, y2, y3, y4, y5, y6 = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    v_lst = [(1, 3),
             (2, 3),
             (3, 3),
             (4, 3),
             (5, 3),
             (6, 3),
             (1, 1),
             (2, 1),
             (3, 1),
             (4, 1),
             (5, 1),
             (6, 1)]

    e_lst = [
        (x1, y1),
        (x1, y4),
        (x2, y1),
        (x2, y2),
        (x2, y5),
        (x3, y2),
	(x3, y3),
	(x3, y6),
        (x4, y3),
        (x5, y6),
        (x6, y5),
    ]

    plot_graph(s_lst, v_lst, e_lst, filename)


def plot_dp(filename="dp.png"):
    count = 7
    s, a, b, c, d, e, t = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    v_lst = [(1, 2),
             (2, 3),
             (4, 3),
             (3, 2),
             (4, 1),
             (2, 1),
             (5, 2)]

    e_lst = [
	(s, a),
	(s, c),
	(s, e),
	(a, b),
	(b, t),
	(c, b),
	(c, d),
	(c, t),
	(d, t),
	(e, c)
    ]

    plot_graph(s_lst, v_lst, e_lst, filename)


def plot_mf(filename="mf.png"):
    count = 6
    s, v1, v2, v3, v4, t = range(count)
    s_lst = [""] * count

    for key, val in locals().items():
        if isinstance(val, int) and 0 <= val < count:
            s_lst[val] = key

    v_lst = [(1, 2),
             (2, 3),
             (2, 1),
             (4, 3),
             (4, 1),
             (5, 2)]

    e_lst = [
        (s, v1, 16),
        (s, v2, 13),
        (v1, v3, 12),
        (v2, v1, 4),
        (v2, v4, 14),
        (v3, v2, 9),
        (v3, t, 20),
        (v4, v3, 7),
        (v4, t, 4)
    ]

    plot_graph(s_lst, v_lst, e_lst, filename)


if __name__ == "__main__":
    plot_scc()
    plot_mst()
    plot_spa(radius=1/3)
    plot_spn()
    plot_mbm()
    plot_dp()
    plot_mf()
