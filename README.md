# line profile

スペクトル線形について


Rustで線幅のない信号に線幅を付ける処理を実装する。結果の可視化には`gnuplot`を呼び出すクレートである
[RustGnuplot](https://github.com/SiegeLord/RustGnuplot)を用いる。

## line profile function

ある線幅のない信号 $(x_i, y_i)$ にline shape 関数 $f(\xi)$ によって幅付けられた信号の関数 $\rho(\xi)$ は以下で表現される。  

$$
\rho(\xi) = \sum_{i} y_i f(\xi - x_i)
$$

実際計算する際には $\xi$ は連続ではなく離散的な値であるのでこれを　$\xi_j$ として離散的な信号 $\rho(\xi_j)$として考える 

$$
\rho(\xi_j) = \sum_{i} y_i f(\xi_j - x_i)
$$

これは $y_i$ と $f(x_i)$ の畳み込みと見ることができる。代表的なline shape関数には Lorentz関数、Gauss関数がある。

### Lorentz線形

Lorentz関数は蛍光寿命による線幅の広がりを表現するのに用いられる関数で、半値全幅を $\Gamma$ としたとき以下の形で表現される。

$$
L(x) = \frac{\Gamma}{2\pi} \frac{1}{(x - x_{centor})^2 + (\Gamma / 2)^2}
$$

### Gauss線形

Gauss関数はドップラー幅を表現するのに用いられる関数で、半値全幅を $\sigma$ としたとき以下の形で表現される。

$$
G(x) = \frac{\sqrt{\ln(2) / \pi}}{\sigma / 2} \exp\left(-\frac{(x - x_{centor})^2}{\sigma/ 2}\right)
$$
