# 数据分析

- [polars](https://docs.pola.rs)
- [statrs](https://github.com/statrs-dev/statrs)Statistical computation library for Rust

## 1. 统计分析

### 1.1 最小值(min)

$$Min = \min_{i=1}^{n}x_i$$

### 1.1.1 最大值(max)

$$Max = \max_{i=1}^{n}x_i$$

### 1.1.2 总和(sum)

$$Sum = \sum_{i=1}^{n}x_i$$

### 1.1.3 平均值(mean)

$$Mean = \frac{1}{n}\sum_{i=1}^{n}x_i$$

### 1.1.4 中位数(median)

如果数据样本是有序的, 中位数是位于中间位置的数值.

- 如果数据样本的数量是奇数, 中位数就是位于中间位置的数值；

$$\text{Median} = x_{\left(\frac{n+1}{2}\right)}$$

- 如果数据样本的数量是偶数, 中位数就是中间两个数值的平均值.
$$\text{Median} = \frac{x_{\left(\frac{n}{2}\right)} + x_{\left(\frac{n}{2} + 1\right)}}{2}$$

### 1.1.5 方差(variance)/标准差(standard deviation)

- 求均值 ($\mu$): 算出所有数字的平均数.
- 算偏差: 用每个数字减去均值.
- 平方处理: 把得到的偏差全部平方(为了消除负号).
- 算平均平方值(方差): 把这些平方后的数字加起来, 除以总个数 $N$.这步得到的结果叫**方差**(Variance).

$$Var = \frac{1}{n}\sum_{i=1}^{n}(x_i - \mu)^2$$

- 开平方根: 对方差开根号, 就得到了**标准差**.

$$SD = \sqrt{Var} = \sqrt{\frac{1}{n}\sum_{i=1}^{n}(x_i - \mu)^2}$$

### 1.1.6 直方图(histogram)

TODO

### 1.1.7 分位数(quantile)

- `nearest` 最近邻法, 取距离计算出的索引i最近的那个整数索引对应的值.  
- `lower` 下确界法, 无论小数部分是多少, 始终向下取整.
- `higher` 上确界法, 无论小数部分是多少, 始终向上取整.
- `midpoint` 中点法, 取i左右两侧相邻两个观测值的算术平均数.
- `linear` 线性插值法, 根据索引的小数部分, 在两个相邻值之间进行比例分配.
- `equiprobable` 等概率法.

### 1.1.8 加权分位数(weighted quantile)
