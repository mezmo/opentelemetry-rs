library 'magic-butler-catalogue'

def DEFAULT_BRANCH = 'main'
def CURRENT_BRANCH = currentBranch()
def WORKSPACE_PATH = "/tmp/workspace/${env.BUILD_TAG.replace('%2F', '/')}"

def CREDS = [
  string(
    credentialsId: 'github-api-token',
    variable: 'GITHUB_TOKEN'
  ),
]
def NPMRC = [
  configFile(fileId: 'npmrc', variable: 'NPM_CONFIG_USERCONFIG')
]

pipeline {
  agent {
    node {
      label 'rust-x86_64'
      customWorkspace(WORKSPACE_PATH)
    }
  }

  options {
    timeout time: 1, unit: 'HOURS'
    timestamps()
    ansiColor 'xterm'
    withCredentials(CREDS)
  }
  environment {
    NPM_CONFIG_CACHE = '.npm'
    SPAWN_WRAP_SHIM_ROOT = '.npm'
    RUSTUP_HOME = '/opt/rust/rustup'
    CARGO_HOME = '/opt/rust/cargo'
    PATH = """${sh(
       returnStdout: true,
       script: 'echo /opt/rust/cargo/bin:\$PATH'
    )}
    // """
  }

  post {
    always {
      jiraSendBuildInfo()
    }
  }

  stages {
    stage('Validate PR Source') {
      when {
        expression { env.CHANGE_FORK }
        not {
          triggeredBy 'issueCommentCause'
        }
      }
      steps {
        error("A maintainer needs to approve this PR for CI by commenting")
      }
    }

    stage('Commitlint and dry release test'){
      tools {
        nodejs 'NodeJS 20'
      }
      environment {
        GIT_BRANCH = "${CURRENT_BRANCH}"
        // This is not populated on PR builds and is needed for the release dry runs
        BRANCH_NAME = "${CURRENT_BRANCH}"
        CHANGE_ID = ""
      }
      steps {
        script {
          configFileProvider(NPMRC) {
            sh 'npm install --ignore-scripts'
            sh 'npm run commitlint'
            sh 'npm run release:dry'
          }
        }
      }
    }

    stage('Unit Tests') {
      steps {
          sh "make clean"
          sh 'make test'
      }
    }
    stage('Clippy') {
      steps {
        // `cargo-audit` is installed on `make test` stage
        sh 'make lint'
      }
    }

    stage('Release') {
      when {
        branch DEFAULT_BRANCH
      }
      tools {
        nodejs 'NodeJS 20'
      }
      steps {
        script {
          configFileProvider(NPMRC) {
            sh 'npm install'
            sh 'npm run release'
          }
        }
      }
    }
  }
}
