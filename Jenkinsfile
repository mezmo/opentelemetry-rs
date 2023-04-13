def DEFAULT_BRANCH = 'main'
def CURRENT_BRANCH = [env.CHANGE_BRANCH, env.BRANCH_NAME]?.find{branch -> branch != null}

def NPMRC = [
  configFile(fileId: 'npmrc', variable: 'NPM_CONFIG_USERCONFIG')
]

pipeline {
  agent {
    node {
      label 'ec2-fleet'
      customWorkspace("/tmp/workspace/${env.BUILD_TAG}")
    }
  }

  options {
    timeout time: 1, unit: 'HOURS'
    timestamps()
    ansiColor 'xterm'
  }
  environment {
    GITHUB_USER = 'jenkins'
    GITHUB_TOKEN = credentials('github-api-token')
    NPM_CONFIG_CACHE = '.npm'
    SPAWN_WRAP_SHIM_ROOT = '.npm'
    RUSTUP_HOME = '/opt/rust/rustup'
    CARGO_HOME = '/opt/rust/cargo'
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL = 'sparse'
    PATH = """${sh(
       returnStdout: true,
       script: 'echo /opt/rust/cargo/bin:\$PATH'
    )}
    """
    // for the semantic-release-rust executable, we must have this set even when not publishing the crate directly
    CARGO_REGISTRY_TOKEN = "not-in-use"
    LAST_COMMITTER = sh(script: 'git log -1 --format=%ae', returnStdout: true).trim()
  }

  tools {
    nodejs 'NodeJS 14'
  }

  post {
    always {
      jiraSendBuildInfo()
    }
  }

  stages {
    stage('Lint and Test') {
      parallel {
        stage('Lint') {
          agent {
            docker {
              label 'ec2-fleet'
              customWorkspace "/tmp/workspace/${BUILD_TAG}"
              image 'us.gcr.io/logdna-k8s/rust:bullseye-1-stable-x86_64'
              reuseNode true
            }
          }
          steps {
            sh 'make lint'
          }
        }
        stage('Unit Tests') {
         agent {
            docker {
              label 'ec2-fleet'
              customWorkspace "/tmp/workspace/${BUILD_TAG}"
              image 'us.gcr.io/logdna-k8s/rust:bullseye-1-stable-x86_64'
              reuseNode true
            }
          }
          steps {
              sh 'make test'
          }
        }
      }
    }

    stage('Release Lint and Test') {
      stages {
        stage('Validate') {
          steps {
            script {
              sh "mkdir -p ${NPM_CONFIG_CACHE}"
              configFileProvider(NPMRC) {
                sh 'npm i && npm run lint'
              }
            }
          }
        }

        stage('Release Test') {
          when {
            not {
              branch 'main'
            }
          }
          environment {
            GIT_BRANCH = "${CURRENT_BRANCH}"
            BRANCH_NAME = "${CURRENT_BRANCH}"
            RUSTUP_HOME = '/opt/rust/cargo'
            CHANGE_ID = ""
          }
          steps {
            script {
              sh "mkdir -p ${NPM_CONFIG_CACHE}"
              sh "unset CARGO_REGISTRIES_CRATES_IO_PROTOCOL; cargo install cargo-edit"
              configFileProvider(NPMRC) {
                sh 'npm i && npm run release:dry'
              }
            }
          }
        }
      }
    }

    stage('Release') {
      when {
        beforeAgent true
        branch DEFAULT_BRANCH
        not {
          changelog '\\[skip ci\\]'
        }
      }
      environment {
        RUSTUP_HOME = '/opt/rust/cargo'
      }
      steps {
        script {
          sh "mkdir -p ${NPM_CONFIG_CACHE}"
          sh "unset CARGO_REGISTRIES_CRATES_IO_PROTOCOL; cargo install cargo-edit"
          configFileProvider(NPMRC) {
            sh 'npm i && npm run release:dry'
          }
        }
      }
    }
  }
}
