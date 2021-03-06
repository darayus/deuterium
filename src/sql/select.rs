use from::{FromSelect};
use select_query::{
    Select,
    SelectQuery, RcSelectQuery,
    SelectFor
};

use sql::{SqlContext, ToSql, QueryToSql};
use sql::value::{ToPredicateValue};
use sql::from::{FromToSql};

impl<T, L, M> FromToSql for FromSelect<T, L, M> {
    fn to_from_sql(&self, ctx: &mut SqlContext) -> String {
        format!("({}) as {}", self.select.to_sql(ctx), self.alias.to_string())
    }
}

impl ToSql for SelectFor {
    fn to_sql(&self, _ctx: &mut SqlContext) -> String {
        match self {
            &SelectFor::Update => "FOR UPDATE",
            &SelectFor::UpdateNoWait => "FOR UPDATE NOWAIT",
            &SelectFor::Share => "FOR SHARE",
            &SelectFor::ShareNoWait => "FOR SHARE NOWAIT",
        }.to_string()
    }
}

impl<T, L, M> ToSql for SelectQuery<T, L, M> {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        let mut sql = "SELECT".to_string();

        if self.distinct.is_some() {
            sql = format!("{} {}", sql, self.distinct.as_ref().unwrap().to_sql(ctx));
        }

        sql = format!("{} {} FROM {}", 
            sql,
            self.select.to_sql(ctx), 
            self.from.as_sql().to_from_sql(ctx)
        );

        if !self.joins.is_empty() {
            let joins: Vec<String> = self.joins.iter().map(|join| join.to_sql(ctx)).collect();
            sql = format!("{} {}", sql, joins.connect(" "))
        }

        if self.where_.is_some() {
            sql = format!("{} WHERE {}", sql, self.where_.as_ref().unwrap().to_sql(false, ctx));
        }

        if self.group_by.is_some() {
            sql = format!("{}{}", sql, self.group_by.as_ref().unwrap().to_sql(ctx));
        }

        if self.having.is_some() {
            sql = format!("{} HAVING {}", sql, self.having.as_ref().unwrap().to_sql(false, ctx));
        }

        if !self.order_by.is_empty() {
            let orders: Vec<String> = self.order_by.iter().map(|ord| ord.to_sql(ctx)).collect();
            sql = format!("{} ORDER BY {}", sql, orders.connect(", "))
        }

        if self.limit.is_some() {
            sql = format!("{} LIMIT {}", sql, self.limit.unwrap())
        }

        if self.offset.is_some() {
            sql = format!("{} OFFSET {}", sql, self.offset.unwrap())
        }

        if self.for_.is_some() {
            sql = format!("{} {}", sql, self.for_.as_ref().unwrap().to_sql(ctx))
        }

        sql
    }
}

impl<T, L, M> QueryToSql for SelectQuery<T, L, M> {}

impl ToSql for RcSelectQuery {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        (**self).to_sql(ctx)
    }
}

impl ToSql for Select {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        match self {
            &Select::Only(ref fields) => {
                let defs: Vec<String> = fields.iter().map(|f| f.expression_as_sql().to_sql(ctx)).collect();
                defs.connect(", ")
            },
            &Select::All => "*".to_string()
        }
    }
}

impl<T, L, M> ToPredicateValue for SelectQuery<T, L, M> {
    fn to_predicate_value(&self, ctx: &mut SqlContext) -> String { self.to_sql(ctx) }   
}