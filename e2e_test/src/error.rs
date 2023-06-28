use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get kube client: {0}")]
    ClientGetFailed(#[from] kube_client::Error),

    #[error("Failed to get CRD: {0}")]
    CRDGetFailed(#[source] kube::Error),

    #[error("Timeout, e2e test failed!")]
    Timeout,

    #[error("Statefulset is not consistent with zookeeper cluster spec!")]
    ZookeeperStsFailed,

    #[error("Statefulset is not consistent with rabbitmq cluster spec!")]
    RabbitmqStsFailed,

    #[error("ConfigMap is not consistent with rabbitmq cluster spec!")]
    RabbitmqConfigMapFailed,

    #[error("Rabbitmq failed to set customized user/password!")]
    RabbitmqUserPassFailed,
}