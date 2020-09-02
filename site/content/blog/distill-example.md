<!--
title: 什么是distill风格的在线学术文章
desc: 本文简单介绍了 distill.pub 网站, distill 风格文章以及 为何要倡导做一名Distiller. 
template: blog
target: artical
date: 2019-10-10
bib: bibliography.bib
-->

### Distill.pub网站介绍

[Distill.pub](https://distill.pub/) 是一家关于机器学习的线上学术期刊, 它由 Google, OpenAI 等企业赞助支持, 
采用开源方式进行文章录用以及编辑, 编委由一批机器学习领域资深的科学家组成. 
Distill.pub 特别之处在于它主要录用所谓 "distill" 风格的文章, 具体解释可以参考, 这篇文章: [学术研究的债务 ](https://distill.pub/2017/research-debt/). 

Distill.pub 的文章特点包括：

* 文章主题, 必须针对的是机器学习领域中较深入的课题, 较复杂的应用
* 文章内容，必须深入浅出，思路清晰且具体，提倡将问题讲透, 讲清楚，偿还所谓的 "Research Debt" 
* 大量采用 HTML5 交互以及高质量的配图, 发挥线上学术文章优势
* 文章排版采用严格的学术论文风格，包括参考文献格式/DOI等
* 期刊采用开源形式发表文章，具体可以参考网站的[github](https://github.com/distillpub)

从上面的这些特点, 可以看出网站录用的既不是知乎体也不是自媒体形式. 网站录用的是严肃的学术科研文章, 拥有严格的编辑和编委, 
文章主题更多是面向学术解释和教学. 

### 什么是 Distiller ?

Distill.pub网站倡导撰写这样的文章，并认为这样的文章具备相当的学术价值. 
[ 学术研究的债务 ](https://distill.pub/2017/research-debt/) 这篇文章鲜明的指出，理论研究者 / 实践工程师以及 **Distiller** 三者都是学术界工作重要的组成者. 
**Distiller** 的价值就在在于，弥补了传统学术文章的缺陷, 加快了理论和技术的普及和推广, 大大减少了学术研究的学习成本. 
因此，Distill.pub 对优秀的distill风格的文章, 提供很高的[奖励](https://distill.pub/prize/ )的. 

## Distill 风格的 Markdown 格式

本个人网站采用Distill.pub开源的前端模板，开发实现了一个定制化的 distill 风格的 Markdown 渲染器. 
网站采用 Rust 语言开发, 集成 tera 模板引擎和 pulldown-cmark 解释器等开源工具. 以下是Markdown格式的各种测试案例. 

### 数学公式

下面是一个独立行数学公式的例子，直接使用\`\`\`^ xxxx \`\`\` 格式，类似Markdown的多行code, 但是在\`\`\`紧跟一个^符号。

一个简单的矩阵的例子：

```^
\begin{matrix}
1 & 0\\
0 & 1
\end{matrix}
```

一个简单的数学公式例子：

```^
F(x) = \int^a_b \frac{1}{3}x^3
```

同样网站也支持行内的数学公式形式，比如`^ c^2 = a^2 + b^2 `这样的形式。原有的Makrdown内置的代码功能，继续支持, 如下面的案例:

```clike
#include <stdio.h>
int main() {    

    int number1, number2, sum;
    
    printf("Enter two integers: ");
    scanf("%d %d", &number1, &number2);

    // calculating sum
    sum = number1 + number2;      
    
    printf("%d + %d = %d", number1, number2, sum);
    return 0;
}
```

### 引用和脚注(footnote)支持

Distill模板支持引用 Citation , 这里采用了\[@xxx\]的扩展，如论文参考[@dong2014image]，多篇参考[@gregor2015draw].

[^note1]: 这是一个简单的单行脚注格式, 自动排版在行文的最后，支持行内`^ c^2 = a^2 + b^2 `数学公式。

扩展了Markdown支持distill风格的脚注[^note1], 也是标准的学术论文形式。

<aside> 这是一个放置在文档右边标注的例子 </aside>

### 图像文件显示的支持 (TODO)

测试一下图像文件的使用

![Image of Yaktocat](https://www.google.com.hk/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png)



### 内置复杂的交互案例 (TODO)

Distill.pub倡导大量应用HTML5可视化交互，这是distill非常显著的特点之一，期刊收入的文章，几乎都有非常专业和漂亮的交互配图. 
本网站也对Markdown进行了内置JavaScript扩展. 

