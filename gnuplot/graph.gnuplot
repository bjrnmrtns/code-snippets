set terminal png notransparent rounded size 800, 600

set xtics nomirror
set ytics nomirror

set mxtics 2
set mytics 2

set style line 80 lt 0 lc rgb "#606060"
set border 3 back ls 80

set style line 81 lt 0 lc rgb "#606060" lw 0.5

set grid xtics
set grid ytics
set grid mxtics
set grid mytics

set grid back ls 81

set arrow from graph 0, first -3 to graph 1, first -3 nohead lw 2 lc rgb "#000000" front

set label "timings" at graph 0.9, first -2

set style line 1 lt 1 lc rgb "#ff0000" lw 2 pt 7 ps 1.5
set style line 2 lt 1 lc rgb "#000000" lw 2 pt 7 ps 1.5

set output "graph-output.png"

set xlabel "iteration"
set ylabel "ms"
set xrange [0:50]
set yrange [0:100]

set title "Graph title"

set key left bottom

plot "graph-input.dat" u 1:2 w lp ls 1 t "line-1", \
"graph-input.dat" u 1:3 w lp ls 2 t "line-2",
