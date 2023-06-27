docker image build -f dockerfile -t forum .
if [ $? -ne 0 ]; then
    echo "Oops image build failed, try again"
    exit 1;
else
    while true; do
        read -p "Clean up any unused objects [y/N] " yn
        case $yn in
            [Yy]* ) docker system prune && exit;;
            [Nn]* ) exit;;
            * ) exit;;
        esac
    done
fi