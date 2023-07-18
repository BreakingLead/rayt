# RayTracing

## Plane

$$
P(x,y,z)\\
A(x_1,y_1,z_1)\\
B(x_2,y_2,z_2)\\
C(x_3,y_3,z_3)\\

\vec{OP} = m \cdot \vec{AB} + n \cdot \vec{AC} +\vec{OA}\\



\begin{cases}
x = m(x_2-x_1)+n(x_3-x_1) + x_1 \\
y = m(y_2-y_1)+n(y_3-y_1) + y_1 \\
z = m(z_2-z_1)+n(z_3-z_1) + z_1 \\
\end{cases}
\to
\begin{cases}
x = mx_2+nx_3 + x_1(1-m-n) \\
y = my_2+ny_3 + y_1(1-m-n) \\
z = mz_2+nz_3 + z_1(1-m-n) \\
\end{cases}


$$
