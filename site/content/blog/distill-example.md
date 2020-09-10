<!--
title: 什么是distill风格的在线学术文章
desc: 本文简单介绍了 distill.pub 网站, distill 风格文章以及 为何要倡导做一名Distiller. 
template: blog
target: artical
date: 2019-10-10
bib: bibliography.bib
sveltes:
  - div: demo-app-container
    app: DemoApp
-->

### Distill.pub网站介绍

[Distill.pub](https://distill.pub/) 是一家关于机器学习的线上学术期刊, 它由 Google, OpenAI 等企业赞助发行, 
期刊采用开源方式进行录用及编辑, 编委由一批领域资深的科学家组成. 
Distill.pub 特别之处在于: 它主要录用所谓 "distill" 风格的文章, 具体解释可以参考这篇文章: [学术研究的债务 ](https://distill.pub/2017/research-debt/). 

Distill.pub 的文章特点包括：

* 文章主题, 针对的是机器学习领域中较深入和复杂的课题
* 文章内容，深入浅出，思路清晰具体，提倡将问题讲透, 偿还所谓的科研债务( Research Debt ) 
* 大量采用 HTML5 交互以及高质量的配图, 发挥线上学术文章优势
* 文章排版采用严格的学术论文风格，包括参考文献格式/DOI等
* 期刊采用开源形式发表文章，每篇文章的编辑记录和源代码都是公开的，具体可以参考网站的 [github](https://github.com/distillpub)

从上面的这些特点可以看出，网站录用的是严肃高质量的学术文章, 并且拥有严格高要求的编辑团队, 明显区别于公众号/知乎/Medium的"科普"内容。这也是Distill.pub 文章精彩的原因，受到了大量的机器学习研究者的关注和追捧。

### 什么是 Distiller ?

Distill.pub 网站倡导撰写类似风格的文章，并认为这样的文章具备相当的学术价值, 网站把那些作者称为 "Distiller".
[ 学术研究的债务 ](https://distill.pub/2017/research-debt/) 这篇文章鲜明的指出，理论研究者 / 科学实践工程师以及 **Distiller** 三者都是学术界工作重要的组成者. 
**Distiller** 的价值就在在于，弥补了传统学术文章的缺陷, 加快了理论和创新的普及, 大大减少了学术研究的学习成本. 
因此，Distill.pub 对优秀的 distill 风格的文章, 提供很不错的[奖励](https://distill.pub/prize/ ). 

## Distill 风格的 Markdown 格式

我的个人网站采用Distill.pub开源的前端模板，开发实现了一个定制化的 distill 风格的 Markdown 渲染器. 
网站采用 Rust 语言开发, 集成 tera 模板引擎和 pulldown-cmark 解释器以及Svelte等开源工具. 以下是Markdown格式的各种测试案例. 

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

Distill模板支持引用 Citation , 这里采用了\[@xxx\]的扩展，如论文参考[@dong2014image]，多篇参考[@dong2014image,gregor2015draw].

[^note1]: 这是一个简单的单行脚注格式, 自动排版在行文的最后，支持行内`^ c^2 = a^2 + b^2 `数学公式。

扩展了Markdown支持distill风格的脚注[^note1], 也是标准的学术论文形式。

<aside> 这是一个放置在文档右边标注的例子 </aside>

### 图像文件显示的支持

测试一下图像文件的使用，统一采用占用独立一行显示方式。

![Image of Google](https://www.google.com.hk/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png) 


### 内置复杂的交互案例

在线文章的最大好处，就是可以充分利用Web强大的交互功能，因此Distill.pub网站录用的文章，都配置了精美的交互可视化小程序。

![](svelte "demo-app-container")


我的网站会利用这套工具，撰写和发布distill风格的个人技术/评论文章，如果你感兴趣的话，可以参考 [源代码](https://github.com/Teaonly/teaonly.github.io) .
