import json
import logging

from typing import List


class BaseModel:
  pass


class Config:
  def get(self, feed1, feeds):
    pass


class OpenAIBCC:
  def parse(self):
    return self


class OpenAIBC:
  def __init__(self):
    self.completions = OpenAIBCC()


class OpenAIB:
  def __init__(self):
    self.chat = OpenAIBC()


class OpenAI:
  def __init__(self, base_url, api_key):
    self.base_url = base_url
    self.api_key = api_key
    self.beta = OpenAIB()


class SgcocTblInfo(BaseModel):
  needsTransformation: bool
  nCols: List[str]
  yCols: List[str]
  colName: str
  valName: str
  reasoning: str


def furnish_sgcoc_tbl_info(exeqy_info, exeqy_hdrs):
  """
  请求LLM处理数据查询结果

  Parameters
  ----------
  exeqy_info: list[dict]
    查询的数据
  exeqy_hdrs: list[str]
    查询的表头

  Returns
  -------
  tuple[list[dict], list[str]] | None
    成功则返回处理后的长格式数据、列名列表
  """
  # TODO: 标识列参考
  sys_pmp = """
你是一个专业的数据结构分析专家。你的任务是分析给定的表格数据，并判断其是否需要从“宽”格式转换为“长”格式（这个过程称为“逆透视”或“Unpivot”），并输出结构化的分析结果。

---

## 1. 核心概念定义

请严格遵循以下定义来分析数据：

*   **标识列**：这些列是数据的“维度”或“描述性信息”。它们回答了“是什么”、“在哪里”、“是谁”等问题。例如：`区域`、`产品名称`、`客户ID`、`保护装置电压等级名称`。它们的值通常是文本、代码或分类数据。
*   **度量列**：这些列是需要被“逆透视”的“数值”或“测量值”。它们的列名通常包含一个“类别”（如`年份`、`月份`）和一个“度量”（如`数量`、`销售额`）。它们的值几乎总是数字。

---

## 2. 输入数据的内容格式

*   *表头*：一个字符串列表。
*   *参考数据*：一个JSON数组，每个元素是一个字典，键在表头中有体现。

---

## 3. 分析任务与指令

请按以下步骤进行分析：

1.  *初步分类*：仔细阅读表头和参考数据，将每一列初步判断为“标识列”或“度量列”。
2.  *参考信息*：有一个非穷尽的参考列表，以下字段**极大概率**是“标识列”。请将此作为强烈参考，但最终判断应基于列名本身的语义和数据类型。
    ```json
    {{KNOWN_ID_COLUMNS_REFERENCE_LIST}}
    ```
3.  *判断是否需要转换*：
    *   如果数据中**存在一个或多个“度量列”**，那么就需要进行“逆透视”转换。
    *   如果数据中**没有任何“度量列”**（即所有列都是“标识列”），或者数据看起来已经是“长”格式，那么就**不需要**转换。
4.  *推断新列名*：
    如果需要转换，请从“度量列”的列名中，推断一个通用的“类别名称”和一个通用的“数值名称”。

    推断逻辑：
    *   *类别名称*：代表列名中**变化**的部分，是这些数值的归属维度。
    *   *数值名称*：代表列名中**不变**的部分，是这些数值本身的含义。如果难以推断，请使用“数值”来表示。

    请参考以下多样化的例子来理解推断过程：
    *   时间维度: ["1月销售额", "2月销售额", "3月销售额"] -> 类别: "月份", 数值: "销售额"
    *   产品维度: ["A产品销量", "B产品销量", "C产品销量"] -> 类别: "产品", 数值: "销量"
    *   区域维度: ["东区用户数", "西区用户数", "南区用户数"] -> 类别: "区域", 数值: "用户数"
    *   状态维度: ["待处理工单", "处理中工单", "已完成工单"] -> 类别: "工单状态", 数值: "工单数"
    *   指标维度: ["成本_类型A", "成本_类型B", "成本_类型C"] -> 类别: "成本类型", 数值: "成本"
5.  *构建新表头*：如果需要转换，新的表头应该是 `[所有标识列] + [推断出的类别名称] + [推断出的数值名称]`。如果不需要转换，新表头就是原始表头。

---

## 4. 输出格式

你必须严格遵守以下JSON格式进行输出，不要包含任何其他文字或解释，不包含代码块标记“```”。

```json
{
  "needsTransformation": true/false,
  "nCols": ["标识列的列名1", "标识列的列名2", "..."],
  "yCols": ["度量列的列名1", "度量列的列名2", "..."],
  "colName": "推断出的类别名称",
  "valName": "推断出的数值名称",
  "reasoning": "简短说明你的判断依据，例如：'发现包含‘数量’的度量列，需要逆透视。' 或 '所有列均为标识列，无需转换。'"
}
```

---

## 5. 输入内容

**表头:**
```json
{{TABLE_HEADERS}}
```

**参考数据:**
```json
{{RAW_DATA_SAMPLE}}
```

  """
  usr_pmp1 = """
请根据以上规则分析以下数据。

---

输入数据为：

*表头*：
```
["保护大类code", "保护装置电压等级名称", "1年及以内数量", "2～6年数量", "7～12年数量"]
```

*参考数据*：
```
[
  {"1年及以内数量": 20, "2～6年数量": 69, "7～12年数量": 131, "保护大类code": 1001, "保护装置电压等级名称": "220kV"},
  {"1年及以内数量": 22, "2～6年数量": 52, "7～12年数量": 89, "保护大类code": 1001, "保护装置电压等级名称": "500kV"},
  {"1年及以内数量": 0, "2～6年数量": 7, "7～12年数量": 8, "保护大类code": 1001, "保护装置电压等级名称": "110kV"}
]
```
  """
  ai_pmp1 = """
{
  "needsTransformation": true,
  "nCols": ["保护大类code", "保护装置电压等级名称"],
  "yCols": ["1年及以内数量", "2～6年数量", "7～12年数量"],
  "colName": "年份",
  "valName": "数量",
  "reasoning": "发现‘1年及以内数量’等度量列，需要逆透视。"
}
  """
  usr_pmp2 = """
请根据规则分析以下数据。

---

输入数据为：

*表头*：
```
["区域", "单位", "负责人"]
```

*参考数据*：
```
[
  {"区域": "东北地区", "单位": 1100, "负责人": "DB0010"},
  {"区域": "东北地区", "单位": 1101, "负责人": "DB00345"},
  {"区域": "华北地区", "单位": 1200, "负责人": "HB21001"},
  {"区域": "华北地区", "单位": 1236, "负责人": "HBDD003"},
  {"区域": "华北地区", "单位": 1289, "负责人": "HB123E"},
]
```
  """
  ai_pmp2 = """
{
  "needsTransformation": false,
  "nCols": ["区域", "单位", "负责人"],
  "yCols": [],
  "colName": "",
  "valName": "",
  "reasoning": "所有列均为标识列，数据已是标准格式，无需转换。"
}
  """
  usr_pmp3 = f"""
请根据规则分析以下数据。

---

输入数据为：

*表头*：
```json
{json.dumps(exeqy_hdrs)}
```

*参考数据*：
```json
{json.dumps(exeqy_info)}
```
  """
  msg_arrs = [
    {"role": "system", "content": sys_pmp},
    {"role": "user", "content": usr_pmp1},
    {"role": "assistant", "content": ai_pmp1},
    {"role": "user", "content": usr_pmp2},
    {"role": "assistant", "content": ai_pmp2},
    {"role": "user", "content": usr_pmp3},
    ]
  try:
    openai_clt = OpenAI(
      base_url=Config.get("LLM_API_BASE_URL", "http://localhost:11433/v1"),
      api_key=Config.get("LLM_API_KEY", "ollama")
      )
    completion = openai_clt.beta.chat.completions.parse(
      model=Config.get("LLM_MODEL", "qwen3:30b-a3b-instruct-2507-q4_K_M"),
      messages=msg_arrs,
      response_format=SgcocTblInfo,
      temperature=0.2,
      top_p=0.8,
      extra_body={"top_k": 20, "min_p": 0.0},
      max_tokens=2048
      )

    reap = completion.choices[0].message
    if reap.parsed:
      # TODO: 调用mutate_sgcoc_tbl_info()
      return reap.parsed.translations
    elif reap.refusal:
      logging.warning(f"LLM refused to generate structured output: {reap.refusal}")
      return None
    else:
      logging.error("LLM did not return a parsable object or a refusal.")
      return None
  except Exception as flaw:
    logging.error(f"An error occurred while calling the LLM API: {flaw}")
    return None


def mutate_sgcoc_tbl_info(exeqy_arrs, exeqy_hdrs, y_cols, col_name, val_name):
  """
  将宽格式数据转换为长格式

  Parameters
  ----------
  exeqy_arrs : list[dict]
    原始的字典列表数据
  exeqy_hdrs: list[str]
    原始的列名列表
  y_cols : list[str]
    需要转换的列名
  col_name : str
    转换后的类别列名
  val_name : str
    转换后的数值列名

  Returns
  -------
  tuple[list[dict], list[str]]
    长格式数据，新的列名列表
  """
  # 按照y_cols为标准
  y_cols = [y_coli for y_coli in y_cols if y_coli in exeqy_hdrs]
  if not y_cols:
    return exeqy_arrs, exeqy_hdrs

  reap_arrs = []
  # 不需要修改的列名，即n_cols
  reap_hdrs = [exeqy_hdri for exeqy_hdri in exeqy_hdrs if exeqy_hdri not in y_cols]

  for exeqy_arri in exeqy_arrs:
    n_info = {reap_hdri: exeqy_arri[reap_hdri] for reap_hdri in reap_hdrs}
    for y_coli in y_cols:
      nub_info = n_info.copy()
      nub_info[col_name] = y_coli
      nub_info[val_name] = exeqy_arri[y_coli]
      reap_arrs.append(nub_info)

  reap_hdrs += [col_name, val_name]
  return reap_arrs, reap_hdrs


if __name__ == "__main__":
  final_data = [
    {
      "13～15年数量": 38,
      "15年以上数量": 63,
      "1年及以内数量": 20,
      "2～6年数量": 69,
      "7～12年数量": 131,
      "保护大类code": 1001,
      "保护装置电压等级名称": "220kV"
      },
    {
      "13～15年数量": 54,
      "15年以上数量": 31,
      "1年及以内数量": 22,
      "2～6年数量": 52,
      "7～12年数量": 89,
      "保护大类code": 1001,
      "保护装置电压等级名称": "500kV"
      },
    {
      "13～15年数量": 8,
      "15年以上数量": 6,
      "1年及以内数量": 0,
      "2～6年数量": 7,
      "7～12年数量": 8,
      "保护大类code": 1001,
      "保护装置电压等级名称": "110kV"
      }
    ]
  ai_anly_info = {
    "needsTransformation": True,
    "nCols": ["保护大类code", "保护装置电压等级名称"],
    "yCols": ["11年及以内数量", "2～6年数量", "7～12年数量", "13～15年数量", "15年以上数量"],
    "colName": "年份",
    "valName": "数量",
    "reasoning": "发现‘...数量’等度量列，需要逆透视。"
    }

  if ai_anly_info["needsTransformation"]:
    y_cols = ai_anly_info.get("yCols")
    if y_cols:
      col_name = ai_anly_info.get("colName")
      if not col_name:
        col_name = "未知合并列"
      val_name = ai_anly_info.get("valName")
      if not val_name:
        val_name = "数量"
      tbl_arrs, tbl_hdrs = mutate_sgcoc_tbl_info(
        final_data, list(final_data[0].keys()), y_cols, col_name, val_name)
      print(tbl_hdrs)
      print(tbl_arrs)
