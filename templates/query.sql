select
    {% for column in columns %}
    {{ column.expression }} as {{ column.name }}{% if ! loop.last %}, {% endif %}
    {% endfor %}
from fact_day_line_item
{% if Dim::LI.isin(joins) %} inner join dim_line_item on dim_line_item.id = fact_day_line_item.line_item_id {% endif %}
{% if Dim::IO.isin(joins) %} inner join dim_insertion_order on dim_insertion_order.id = dim_line_item.id {% endif %}
{% if Dim::CA.isin(joins) %} inner join dim_campaign on dim_insertion_order.id = dim_line_item.id {% endif %}
