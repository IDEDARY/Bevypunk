# Git LFS

Game development is not really well suited for Git. The game assets like scenes, 3D models and textures are really big in disk size compared to regular source code. This means that once you start tracking them, they will inflate your Git history really quickly, even if you delete them later. This makes working with the repository really slow and clunky.

To address this issue, there is a Git extension called Git Large File Storage, which handles selected files differently than your source code, making your `git clone`s faster and your Git history cleaner. Good news is, is that it is built-in into all popular Git hosting sites.

If your file was already being tracked the normal way, it will stay in the history even after you add it to LFS. You have to use LFS from the start of the repository.

## Install

You need to have it installed on all your machines first.

```
sudo apt install git-lfs
```

## New repository

If you start new repository, just run this command to enable Git-LFS

```
git lfs install
```

## Tracking

You can then add this naming rule to use LFS for all matching files.

```
git lfs track "*.png"
```

or manually add this line to `.gitattributes`

```
*.png filter=lfs diff=lfs merge=lfs -text
```
