use super::Workbook;
use super::Format;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConditionalFormatType {
    Cell,
    Blanks
}

impl ConditionalFormatType {
    pub fn value(self) -> u8 {
        let value = match self {
            ConditionalFormatType::Cell => {
                libxlsxwriter_sys::lxw_conditional_format_types_LXW_CONDITIONAL_TYPE_CELL
            },
            ConditionalFormatType::Blanks => {
                libxlsxwriter_sys::lxw_conditional_format_types_LXW_CONDITIONAL_TYPE_BLANKS
            },
        };
        value as u8
    }
}

pub enum ConditionalFormatCriteria {
    Between,
    EqualTo,
    LessThan,
    NotBetween,
    GreaterThan,
    NotEqualTo,
}

impl ConditionalFormatCriteria {
    pub fn value(self) -> u8 {
        let value = match self {
            ConditionalFormatCriteria::Between => {
                libxlsxwriter_sys::lxw_conditional_criteria_LXW_CONDITIONAL_CRITERIA_BETWEEN
            },
            ConditionalFormatCriteria::EqualTo => {
                libxlsxwriter_sys::lxw_conditional_criteria_LXW_CONDITIONAL_CRITERIA_EQUAL_TO
            },
            ConditionalFormatCriteria::LessThan => {
                libxlsxwriter_sys::lxw_conditional_criteria_LXW_CONDITIONAL_CRITERIA_LESS_THAN
            },
            ConditionalFormatCriteria::NotBetween => {
                libxlsxwriter_sys::lxw_conditional_criteria_LXW_CONDITIONAL_CRITERIA_NOT_BETWEEN
            },
            ConditionalFormatCriteria::GreaterThan => {
                libxlsxwriter_sys::lxw_conditional_criteria_LXW_CONDITIONAL_CRITERIA_GREATER_THAN
            },
            ConditionalFormatCriteria::NotEqualTo => {
                libxlsxwriter_sys::lxw_conditional_criteria_LXW_CONDITIONAL_CRITERIA_NOT_EQUAL_TO
            },
        };
        value as u8
    }
}

pub struct ConditionalFormat<'a> {
    pub(crate) _workbook: &'a Workbook,
    pub(crate) conditional_format: libxlsxwriter_sys::lxw_conditional_format,
}


impl<'a> ConditionalFormat<'a> {

    pub fn new(workbook: &'a Workbook) -> Self {
        ConditionalFormat { _workbook: workbook,
            conditional_format: libxlsxwriter_sys::lxw_conditional_format {
                type_: 0,
                criteria: 0,
                value: 0.0,
                value_string: std::ptr::null_mut::<i8>(),
                format: std::ptr::null_mut::<libxlsxwriter_sys::lxw_format>(),
                min_value: 0.0,
                min_value_string: std::ptr::null_mut::<i8>(),
                min_rule_type: 0,
                min_color: 0,
                mid_value: 0.0,
                mid_value_string: std::ptr::null_mut::<i8>(),
                mid_rule_type: 0,
                mid_color: 0,
                max_value: 0.0,
                max_value_string: std::ptr::null_mut::<i8>(),
                max_rule_type: 0,
                max_color: 0,
                bar_color: 0,
                bar_only: 0,
                data_bar_2010: 0,
                bar_solid: 0,
                bar_negative_color: 0,
                bar_border_color: 0,
                bar_negative_border_color: 0,
                bar_negative_color_same: 0,
                bar_negative_border_color_same: 0,
                bar_no_border: 0,
                bar_direction: 0,
                bar_axis_position: 0,
                bar_axis_color: 0,
                icon_style: 0,
                reverse_icons: 0,
                icons_only: 0,
                multi_range: std::ptr::null_mut::<i8>(),
                stop_if_true: 0
            }
        }
    }

    pub fn set_type(mut self, cf_type: ConditionalFormatType) -> Self {
        self.conditional_format.type_ = cf_type.value(); 
        self
    }

    pub fn set_criteria(mut self, cf_criteria: ConditionalFormatCriteria) -> Self {
        self.conditional_format.criteria = cf_criteria.value();
        self
    }

    pub fn set_value(mut self, value: f64) -> Self {
        self.conditional_format.value = value;
        self
    }

    pub fn set_min_value(mut self, value: f64) -> Self {
        self.conditional_format.min_value = value;
        self        
    }

    pub fn set_max_value(mut self, value: f64) -> Self {
        self.conditional_format.max_value = value;
        self        
    }

    pub fn set_format(mut self, format: &Format) -> Self {
        self.conditional_format.format = format.format;
        self
    }
}
