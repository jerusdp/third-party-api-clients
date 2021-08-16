use anyhow::Result;

use crate::Client;

pub struct Orgunits {
    client: Client,
}

impl Orgunits {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Orgunits { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/orgunits` endpoint.
     *
     * Retrieves a list of all organizational units for an account.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `org_unit_path: &str` -- The full path to the organizational unit or its unique ID. Returns the children of the specified organizational unit.
     * * `type_: crate::types::DirectoryOrgunitsListType` -- Whether to return all sub-organizations or just immediate children.
     */
    pub async fn directory_list(
        &self,
        customer_id: &str,
        org_unit_path: &str,
        type_: crate::types::DirectoryOrgunitsListType,
    ) -> Result<crate::types::OrgUnits> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !org_unit_path.is_empty() {
            query_args.push(format!("org_unit_path={}", org_unit_path));
        }
        query_args.push(format!("type={}", type_));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/orgunits?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/orgunits` endpoint.
     *
     * Adds an organizational unit.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     */
    pub async fn directory_insert(
        &self,
        customer_id: &str,
        body: &crate::types::OrgUnit,
    ) -> Result<crate::types::OrgUnit> {
        let url = format!(
            "/admin/directory/v1/customer/{}/orgunits",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/orgunits/{orgUnitPath}` endpoint.
     *
     * Retrieves an organizational unit.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `org_unit_path: &str` -- The full path of the organizational unit or its unique ID.
     */
    pub async fn directory_get(
        &self,
        customer_id: &str,
        org_unit_path: &str,
    ) -> Result<crate::types::OrgUnit> {
        let url = format!(
            "/admin/directory/v1/customer/{}/orgunits/{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&org_unit_path.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customerId}/orgunits/{orgUnitPath}` endpoint.
     *
     * Updates an organizational unit.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `org_unit_path: &str` -- The full path of the organizational unit or its unique ID.
     */
    pub async fn directory_update(
        &self,
        customer_id: &str,
        org_unit_path: &str,
        body: &crate::types::OrgUnit,
    ) -> Result<crate::types::OrgUnit> {
        let url = format!(
            "/admin/directory/v1/customer/{}/orgunits/{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&org_unit_path.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customerId}/orgunits/{orgUnitPath}` endpoint.
     *
     * Removes an organizational unit.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `org_unit_path: &str` -- The full path of the organizational unit or its unique ID.
     */
    pub async fn directory_delete(&self, customer_id: &str, org_unit_path: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/customer/{}/orgunits/{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&org_unit_path.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customerId}/orgunits/{orgUnitPath}` endpoint.
     *
     * Updates an organizational unit. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch)
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `org_unit_path: &str` -- The full path of the organizational unit or its unique ID.
     */
    pub async fn directory_patch(
        &self,
        customer_id: &str,
        org_unit_path: &str,
        body: &crate::types::OrgUnit,
    ) -> Result<crate::types::OrgUnit> {
        let url = format!(
            "/admin/directory/v1/customer/{}/orgunits/{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&org_unit_path.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
