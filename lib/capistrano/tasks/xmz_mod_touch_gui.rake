namespace :deploy do

  desc "Compile xMZ-Mod-Touch-Gui"
  task :compile do
    on roles(:app) do
      execute "cd #{fetch(:deploy_to)}/current; sh ./autogen.sh"
      execute "cd #{fetch(:deploy_to)}/current; sh ./configure \
        --prefix=/usr"
      execute "cd #{fetch(:deploy_to)}/current; make"
    end
  end

  desc "Install xMZ-Mod-Touch-Gui"
  task :make_install do
    on roles(:app) do
      execute "cd #{fetch(:deploy_to)}/current; make install"
    end
  end

  desc "Start installation"
  task :build do
    invoke "deploy:compile"
    #invoke "xmz_mod_touch_gui:make_tests"
    invoke "deploy:make_install"
  end


  after :published, :build do
  end

  after :build, :restart do
    on roles(:app) do
      execute "service xmz restart"
    end
  end


  namespace :weston do
    desc "Stop weston display server"
    task :stop do
      on roles(:app) do
        execute "service weston stop"
      end
    end

    desc "Start weston display server"
    task :start do
      on roles(:app) do
        execute "service weston start"
      end
    end

    desc "Restart weston display server"
    task :restart do
      on roles(:app) do
        execute "service weston restart"
      end
    end
  end

end

