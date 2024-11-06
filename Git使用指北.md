#Git使用指北
## 远程更新到本地
### 1.远程代码更新方案
1.查看远程分支，和上面的第一步相同
git remote -v
2.从远程获取最新版本到本地
git fetch origin master
3.比较本地的仓库和远程参考的区别
git log -p master… origin/master
4.把远程下载下来的代码合并到本地仓库，远程的和本地的合并
git merge origin/master
### 2.远程代码下载到本地新建分支；对比区别后在合并）
1.查看远程分支
git remote -v
2.从远程获取最新版本到本地
git fetch origin dev:temp
git fetch origin dev:temp 这句命令的意思是：从远程的origin仓库的dev分支下载到本地并新建一个分支temp
3.较本地的仓库和远程参考的区别
git diff temp
$ git diff temp
命令的意思是：比较dev分支和temp分支的不同
4.合并temp分支到dev分支
git merge temp
5.如果不想要temp分支了，可以删除此分支
$ git branch -d temp
Deleted branch temp (was c54244c).

## 本地更新到远程
1. 将文件添加给git管理
使用git add . （. 表示所有的）或者 git add + 文件名 // 将文件保存到缓存区

git add .

2. 提交文件
使用git commit -m ‘新添加的文件内容描述’

git commit -m '测试提交' //添加文件描述

3. 推送到远程仓库
使用git push origin master ，将本地仓库推送到远程仓库

git push origin master 
#背景
团队其他成员修改了某文件并已提交入库，你在pull之前修改了本地该文件，等你修改完代码再pull时，这时会报错如下错误：
error: Your local changes to the following files would be overwritten by merge
#二、解决方案
根据是否要保存本地修改，有以下两种解决方案

##2.1  保留修改
执行以下三条命令

git stash #封存修改
git pull origin master 
git stash pop #把修改还原
注：
git stash：备份当前工作区内容，从最近的一次提交中读取相关内容，让工作区保证和上次提交的内容一致。同时，将当前工作区内容保存到Git栈中
git pull：拉取服务器上当前分支代码
git stash pop：从Git栈中读取最近一次保存的内容，恢复工作区相关内容。同时，用户可能进行多次stash操作，需要保证后stash的最先被取到，所以用栈（先进后出）来管理；pop取栈顶的内容并恢复
git stash list：显示Git栈内的所有备份，可以利用这个列表来决定从那个地方恢复。
git stash clear：清空Git栈
##2.2 废弃修改
核心思想就是版本回退，具体命令如下
git reset --hard 
git pull origin master
注：不建议使用第二种。除非你再三确定不需要本地的修改了。