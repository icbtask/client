# ICBTask official cli

## 🔌 Installation

You can download the prebuilt binary for your platform in github releases page [here](https://github.com/icbtask/client/releases)

## 🚀 Usage

### Requirements

To be able to use the cli, you'll need an API Key

```
export API_KEY=<Your API Key>
```

You can generate one in your [profile](https://icbtask.com)

### ⚽ Examples

#### Create a todolist

```
$ icbtask todolist add --name="My todolist"
New todolist `My todolist` created
```

#### List all the todolists

```
$ icbtask todolist list
+----------+-------------+
| id       | name        |
+----------+-------------+
| 6a9f48df | My todolist |
+----------+-------------+
```

#### Create a task

```
$ icbtask task add --todolist-id=6a9f48df --project="Blog" --description="Create a new blog post"
New task created
```

#### List all tasks

```
$ icbtask task list
+----------+---------+------------------------+
|               My todolist                   |
+----------+---------+------------------------+
|    id    | project |      description       |
+----------+---------+------------------------+
| 97ea5748 |  Blog   | Create a new blog post |
+----------+---------+------------------------+
```

#### Create a new address

```
$ icbtask address add
New address created
```

#### List all the addresses

```
$ icbtask address list
+------------------------------------------------------+-------------+-------------------+
| address                                              | todolist_id | allowed_addresses |
+------------------------------------------------------+-------------+-------------------+
| hnqt34rmq7lokybzcnqzzhwcjaxmnhmr4ef5g4ufj3lthutn7xpa |             |                   |
+------------------------------------------------------+-------------+-------------------+
```

#### Attach an address to a todolist

```
$ icbtask address attach --address=hnqt34rmq7lokybzcnqzzhwcjaxmnhmr4ef5g4ufj3lthutn7xpa --todolist-id=6a9f48df
Address attached successfuly

$ icbtask address list
+------------------------------------------------------+-------------+-------------------+
| address                                              | todolist_id | allowed_addresses |
+------------------------------------------------------+-------------+-------------------+
| hnqt34rmq7lokybzcnqzzhwcjaxmnhmr4ef5g4ufj3lthutn7xpa | 6a9f48df    |                   |
+------------------------------------------------------+-------------+-------------------+
```

#### Allow an address to send you tasks

```
$ icbtask address allow \
    --address=hnqt34rmq7lokybzcnqzzhwcjaxmnhmr4ef5g4ufj3lthutn7xpa \
    --remote-address=uwi7uk5aurxvsqyqvl7qhhnxt57ejn77jxemdqfquraucvm4vu5a
Remote address allowed

$ icbtask address list
+------------------------------------------------------+-------------+-----------------------------------------------------------+
| address                                              | todolist_id | allowed_addresses                                         |
+------------------------------------------------------+-------------+-----------------------------------------------------------+
| hnqt34rmq7lokybzcnqzzhwcjaxmnhmr4ef5g4ufj3lthutn7xpa | 6a9f48df    | uwi7uk5aurxvsqyqvl7qhhnxt57ejn77jxemdqfquraucvm4vu5a @Bob |
|                                                      |             |                                                           |
+------------------------------------------------------+-------------+-----------------------------------------------------------+
```

#### Share a task with an address

```
$ icbtask task share --task-id=97ea5748 --address=lk6v6monyielffwylz4vimn56dkppjrsujo4yp4nnunyijm3fyoa
Task shared

$ icbtask task list
+----------+---------+------------------------+-------------+-------------+
|                               My todolist                               |
+----------+---------+------------------------+-------------+-------------+
|    id    | project |      description       | shared from | shared with |
+----------+---------+------------------------+-------------+-------------+
| 97ea5748 |  Blog   | Create a new blog post |             |    @Bob     |
+----------+---------+------------------------+-------------+-------------+
```

From Bob perspective

```
$ icbtask task list
+----------+---------+------------------------+-------------+-------------+
|                              Bob todolist                               |
+----------+---------+------------------------+-------------+-------------+
|    id    | project |      description       | shared from | shared with |
+----------+---------+------------------------+-------------+-------------+
| 97ea5748 |  Blog   | Create a new blog post |    @Badr    |             |
+----------+---------+------------------------+-------------+-------------+
```

## ✍️ Author

Badr BADRI @pythops

## ⚖️ License

GPLv3
