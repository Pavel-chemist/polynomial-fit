struct PIXC //pixel coordinate
{
    float x;
    float y;
};



float *PolynomialFit ( PIXC *p_coordinates, int length )
{
//	given number of coordinate pairs, output list of coefficients corresponding to coord pair number
    //length should not be less than 2 (?)
    int matrixSize = (length+1)*length;
//	int order = length - 1;
    
    
    float *p_coefficients = new float [length]; //number of coefficients equals number of coordinate pairs given
    
    for ( int i = 0; i < length; i++ )
    {
        p_coefficients[i] = 0;
    }
    
    //create matrix to be processed to Row Echelon Form
    //length+1 by length
    double *p_matrix = new double [matrixSize];
    /*
    
    Matrix to solve:
    
    1		X1		X1^2	Y1			1	0	0	C
    1		X2		X2^2	Y2	===>	0	1	0	B
    1		X3		X3^2	Y3			0	0	1	A
    
    */
    
    for ( int j = 0; j < length; j++ ) //matrix height
    {
        for ( int i = 0; i < length; i++ )
        {
            p_matrix[j*(length+1) + i] = Exp( p_coordinates[j].x, i );
        }
        p_matrix[j*(length+1) + length] = p_coordinates[j].y;
    }
    
    RowEchelonForm ( p_matrix, length, length+1 );
    
    for ( int i = 0; i < length; i++ )
    {
        p_coefficients[length - 1 - i] = p_matrix[i*(length+1) + length];
    }
    
    
    return p_coefficients;
}

float *CalculateParabolicCoefficients ( float *p_coordinates, int length )
{
    float	X1 = p_coordinates[0],
            X2 = p_coordinates[1],
            X3 = p_coordinates[2];
    
    int numOfTriples = length/3 - 1;
    /*
    
    Matrix to solve:
    
    X1^2	X1		1		Y1
    X2^2	X2		1		Y2
    X3^2	X3		1		Y3
    
    */
    
    double *p_matrix = new double [12];
    float intermediate = 0;
    float *p_coefficients = new float [4*numOfTriples];
    //baseline; A; B; C
    
//	
    
    for ( int k = 0; k < numOfTriples; k++ )
    {	
        p_matrix[0] = X1 * X1;
        p_matrix[1] = X1;
        p_matrix[2] = 1;
        p_matrix[3] = p_coordinates[(k+1)*3 + 0];
        p_matrix[4] = X2 * X2;
        p_matrix[5] = X2;
        p_matrix[6] = 1;
        p_matrix[7] = p_coordinates[(k+1)*3 + 1];
        p_matrix[8] = X3 * X3;
        p_matrix[9] = X3;
        p_matrix[10] = 1;
        p_matrix[11] = p_coordinates[(k+1)*3 + 2];
        
        
        RowEchelonForm ( p_matrix, 3, 4 );

        p_coefficients[k*4+0] = p_coordinates[(k+1)*3 + 1];
        intermediate = p_matrix[3];
        p_coefficients[k*4+1] = intermediate;
        intermediate = p_matrix[7];
        p_coefficients[k*4+2] = intermediate;
        intermediate = p_matrix[11] - p_coordinates[(k+1)*3 + 1];
        p_coefficients[k*4+3] = intermediate;
    }
    
    delete[] p_matrix;
    p_matrix = nullptr;
    
    return p_coefficients;
}


void RowEchelonForm ( double* p_data, int rows, int columns )
{
    NormalizeRows ( p_data, rows, columns );
        
    for ( int i = 0; i < rows-1; i++ )
    {
        SubtractRowDown ( p_data, rows, columns, i );
        NormalizeRows ( p_data, rows, columns );
    }
    
    for ( int i = rows-1; i >= 0; i-- )
    {
        SubtractRowUp (p_data, rows, columns, i );
    }
    
    for ( int i = 0; i < rows*columns; i++ )
    {
        if ( p_data[i] == -0.0 )
        {
            p_data[i] = 0;
        }
    }
}

void NormalizeRows ( double* p_data, int rows, int columns )
{
    double leadingValue = 0;
    int count;
    
    for ( int j = 0; j < rows; j++ )
    {
        count = 0;
        leadingValue = 0;
        while ( leadingValue == 0 )
        {			
            leadingValue = p_data[j*columns + count];
            count++;
        }
        for ( int i = 0; i < columns; i++ )
        {
            p_data[j*columns+i] = p_data[j*columns+i] / leadingValue;
        }
    }
    
}

void SubtractRowDown ( double* p_data, int rows, int columns, int rowToSubtract )
{
    for ( int j = rowToSubtract + 1; j < rows; j++ )
    {
        for ( int i = 0; i < columns; i++ )
        {
            p_data[j*columns+i] = p_data[j*columns+i] - p_data[rowToSubtract*columns+i];
        }
    }
}

void SubtractRowUp ( double* p_data, int rows, int columns, int rowToSubtract )
{
    //find first non-zero value index
    int leadingValueIndex = 0;
    double coefficient = 1;
    
    for ( int i = 0; i < columns; i++ )
    {
        if ( p_data[rowToSubtract*columns+i] != 0 )
        {
            leadingValueIndex = i;
            break;
        }
    }
    
    //subtracting rows such that on leadingValueIndex result become zero
    for ( int j = rowToSubtract-1; j >= 0; j -- )
    {
        coefficient = p_data[j*columns+leadingValueIndex] / p_data[rowToSubtract*columns+leadingValueIndex];
        for ( int i = 0; i < columns; i++ )
        {
            p_data[j*columns+i] = p_data[j*columns+i] - coefficient * p_data[rowToSubtract*columns+i];
        }
    }
    
}

double Exp ( double n, int e )
{
    double res = 1.0;
    
    if ( e >= 0 )
    {
        for ( int i = 0; i < e; i++ )
        {
            res *= n;
        }
    }
    else 
    {
        for ( int i = 0; i > e; i-- )
        {
            res = res / n;
        }
    }
    
    return res;
}
