mkdir $HOME/.fav
cd $HOME/.fav
wget https://raw.githubusercontent.com/avjves/Favdir/1.0.0/src/favdir_func.sh .
wget https://github.com/avjves/Favdir/releases/download/1.0.0/favdir .
chmod 775 favdir
echo "source $HOME/.fav/favdir_func.sh" >> $HOME/.bashrc
source $HOME/.bashrc
