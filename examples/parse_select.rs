// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(clippy::all)]

use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::*;

fn main() {
    let sql = "create table a(id int(10) auto_increment primary key CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci comment 'agdd')engine=innodb auto_increment=1 default charset=utf8 COLLATE =utf8mb4_general_ci";
    //let sql = "";
    let dialect = MySqlDialect {};

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:?}", ast);

}
